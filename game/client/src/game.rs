// mochou-p/game/game/client/src/game.rs

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use ggez::{glam, winit};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::graphics::{Canvas, Color, Image, Rect, Text, TextAlign, TextFragment, TextLayout};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};
use glam::{vec2, Vec2};
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::TryRecvError;
use super::network::{ClientMessage, ServerMessage};


pub static ONLINE: AtomicBool = AtomicBool::new(false);

fn init() -> GameResult<(Context, EventLoop<()>)> {
    let title = String::from("game");

    ContextBuilder::new(&title, "mochou-p")
        .with_conf_file(false)
        .window_setup(WindowSetup {
            title,
            samples: NumSamples::One,
            vsync:   true,
            icon:    String::new(),
            srgb:    true
        })
        .window_mode(WindowMode {
            width:                         800.0,
            height:                        600.0,
            min_width:                     1.0,
            min_height:                    1.0,
            max_width:                     0.0,
            max_height:                    0.0,
            maximized:                     false,
            fullscreen_type:               FullscreenType::Windowed,
            borderless:                    false,
            resizable:                     true,
            visible:                       true,
            transparent:                   false,
            resize_on_scale_factor_change: false,
            logical_size:                  None
        })
        .build()
}

pub fn run(g2n_w: Sender<ClientMessage>, n2g_r: Receiver<ServerMessage>) {
    match init() {
        Ok((mut ctx, event_loop)) => {
            match Game::new(&mut ctx, g2n_w, n2g_r) {
                Ok(mut state) => {
                    state.network_queue.push_back(ClientMessage::Tcp(
                        game_protocol::tcp::ClientToServer::LetsShakeHands
                    ));

                    if let Err(game_error) = event::run(ctx, event_loop, state) {
                        utils::error!("game crashed: {game_error}");
                    }
                },
                Err(game_error) => {
                    utils::error!("failed to create game state: {game_error}");
                }
            }
        },
        Err(game_error) => {
            utils::error!("failed to initialize ggez: {game_error}");
        }
    }
}

struct Game {
    g2n_w:                 Sender<ClientMessage>,
    n2g_r:                 Receiver<ServerMessage>,
    network_token:         Option<game_protocol::Token>,
    network_queue:         VecDeque<ClientMessage>,
    network_ready:         bool,
    players:               HashMap<game_core::PlayerId, Vec2>,
    camera_follows_player: bool,
    window_size:           Vec2,
    image1:                Image,
    image2:                Image,
    image_offset:          Vec2,
    text:                  Text,
    text_offset:           Vec2,
    position:              Vec2,
    movement:              Vec2,
    speed:                 f32
}

impl Game {
    pub fn new(
        ctx:   &mut Context,
        g2n_w: Sender<ClientMessage>,
        n2g_r: Receiver<ServerMessage>
    ) -> GameResult<Self> {
        let     image1       = Image::from_bytes(ctx, include_bytes!("../assets/images/player.png"))?;
        let     image2       = Image::from_bytes(ctx, include_bytes!("../assets/images/other.png"))?;
        // TODO: assuming they are the same size
        let     image_width  = image1. width() as f32;
        let     image_height = image1.height() as f32;
        let     image_offset = vec2(image_width * 0.5, image_height * 0.5);
        let mut text         = Text::new(TextFragment { text: String::from("you"), ..Default::default() });
        let     text_offset  = vec2(0.0, image_height * 0.75);

        text.set_layout(TextLayout { h_align: TextAlign::Middle, v_align: TextAlign::Middle });

        Ok(Self {
            g2n_w,
            n2g_r,
            network_token:         None,
            network_queue:         VecDeque::with_capacity(32),
            network_ready:         false,
            players:               HashMap::new(),
            camera_follows_player: false,
            window_size:           Vec2::from(ctx.gfx.drawable_size()),
            image1,
            image2,
            image_offset,
            text,
            text_offset,
            position:              Vec2::ZERO,
            movement:              Vec2::ZERO,
            speed:                 200.0
        })
    }

    fn network_recv(&mut self) {
        loop {
            if !ONLINE.load(Ordering::Relaxed) {
                return;
            }

            match self.n2g_r.try_recv() {
                Ok(message) => {
                    match message {
                        ServerMessage::Tcp(tcp_message) => match tcp_message {
                            game_protocol::tcp::ServerToClient::Handshake { token } => {
                                self.network_token = Some(token);

                                self.network_queue.push_back(ClientMessage::Udp(
                                    game_protocol::udp::ClientToServer::TokenConfirmation { token }
                                ));
                            },
                            game_protocol::tcp::ServerToClient::PlayerExists { id, position } => {
                                self.players.insert(id, Vec2::from_array(position.0));
                            },
                            game_protocol::tcp::ServerToClient::PlayerJoined { id } => {
                                self.players.insert(id, Vec2::ZERO);
                            },
                            game_protocol::tcp::ServerToClient::PlayerLeft { id } => {
                                self.players.remove(&id);
                            }
                        },
                        ServerMessage::Udp(udp_message) => match udp_message {
                            game_protocol::udp::ServerToClient::TokenConfirmed => {
                                self.network_ready = true;
                            },
                            game_protocol::udp::ServerToClient::TokenChallenge { nonce } => {
                                if let Some(token) = self.network_token {
                                    self.network_queue.push_back(ClientMessage::Udp(
                                        game_protocol::udp::ClientToServer::TokenChallenge { token, nonce }
                                    ));
                                } else {
                                    utils::warning!("server asked for a token, but i dont have it");
                                }
                            },
                            game_protocol::udp::ServerToClient::PlayerPosition { id, position } => {
                                if let Some(player) = self.players.get_mut(&id) {
                                    player.x = position.0[0];
                                    player.y = position.0[1];
                                }
                            }
                        }
                    }
                },
                Err(err) => {
                    if matches!(err, TryRecvError::Disconnected) {
                        utils::error!("n2g channel closed");
                        ONLINE.store(false, Ordering::Relaxed);
                    }

                    break;
                }
            }
        }
    }

    fn network_send(&mut self) {
        if !ONLINE.load(Ordering::Relaxed) {
            return;
        }

        while let Some(message) = self.network_queue.pop_front() {
            if let Err(err) = self.g2n_w.try_send(message) {
                utils::error!("g2n channel try_send failed: {err}");
                continue;
            }
        }
    }

    fn network_enqueue(&mut self, message: ClientMessage) {
        if ONLINE.load(Ordering::Relaxed) && self.network_ready {
            self.network_queue.push_back(message);
        }
    }

    fn camera(&self) -> Rect {
        let mut top_left = -self.window_size * 0.5;

        if self.camera_follows_player {
            top_left += self.position;
        }

        Rect::new(top_left.x, top_left.y, self.window_size.x, self.window_size.y)
    }

    fn update_world(&mut self, ctx: &mut Context) {
        let fixed_movement = self.movement
            .clamp(Vec2::NEG_ONE, Vec2::ONE)
            .try_normalize()
            .unwrap_or(Vec2::ZERO);

        if fixed_movement == Vec2::ZERO {
            return;
        }

        self.position += fixed_movement * self.speed * ctx.time.delta().as_secs_f32();

        self.network_enqueue(ClientMessage::Udp(
            game_protocol::udp::ClientToServer::PositionChanged {
                position: game_core::PlayerPosition(self.position.to_array())
            }
        ));
    }

    fn draw_world(&self, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(self.camera());

        for position in self.players.values() {
            canvas.draw(&self.image2, position - self.image_offset);
        }

        canvas.draw(&self.image1, self.position - self.image_offset);
        canvas.draw(&self.text,   self.position - self. text_offset);
    }

    fn draw_screen(&self, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(Rect::one());

        // ui
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.network_recv();
        self.update_world(ctx);
        self.network_send();

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if repeated {
            return Ok(());
        }

        match input.event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) => {
                ctx.request_quit();
            },
            PhysicalKey::Code(KeyCode::KeyW | KeyCode::ArrowUp) => {
                self.movement.y -= 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyS | KeyCode::ArrowDown) => {
                self.movement.y += 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyA | KeyCode::ArrowLeft) => {
                self.movement.x -= 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyD | KeyCode::ArrowRight) => {
                self.movement.x += 1.0;
            },
            _ => ()
        }

        Ok(())
    }

    fn key_up_event(&mut self, _: &mut Context, input: KeyInput) -> GameResult {
        match input.event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW | KeyCode::ArrowUp) => {
                self.movement.y += 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyS | KeyCode::ArrowDown) => {
                self.movement.y -= 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyA | KeyCode::ArrowLeft) => {
                self.movement.x += 1.0;
            },
            PhysicalKey::Code(KeyCode::KeyD | KeyCode::ArrowRight) => {
                self.movement.x -= 1.0;
            },
            _ => ()
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgba(75, 75, 75, 255));

        self.draw_world (&mut canvas);
        self.draw_screen(&mut canvas);

        canvas.finish(ctx)
    }

    fn resize_event(&mut self, _: &mut Context, width: f32, height: f32) -> GameResult {
        self.window_size = vec2(width, height);

        Ok(())
    }
}

