// mochou-p/game/game/client/src/main.rs

use ggez::glam::{vec2, Vec2};
use ggez::winit::keyboard::PhysicalKey;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::graphics::{Canvas, Color, Image, Rect, Text, TextAlign, TextFragment, TextLayout};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput};


fn main() {
    let title = String::from("game");

    let (mut ctx, event_loop) = ContextBuilder::new(&title, "mochou-p")
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
        .unwrap();

    let game = Game::new(&mut ctx);

    event::run(ctx, event_loop, game).unwrap();
}

struct Game {
    camera_follows_player: bool,
    window_size:           Vec2,
    image:                 Image,
    image_offset:          Vec2,
    text:                  Text,
    text_offset:           Vec2,
    position:              Vec2,
    movement:              Vec2,
    speed:                 f32
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let     image        = Image::from_bytes(ctx, include_bytes!("../assets/images/player.png")).unwrap();
        let     image_width  = image. width() as f32;
        let     image_height = image.height() as f32;
        let     image_offset = vec2(image_width * 0.5, image_height * 0.5);
        let mut text         = Text::new(TextFragment { text: String::from("you"), ..Default::default() });
        let     text_offset  = vec2(0.0, image_height * 0.75);

        text.set_layout(TextLayout {
            h_align: TextAlign::Middle,
            v_align: TextAlign::Middle
        });

        Self {
            camera_follows_player: false,
            window_size:           Vec2::from(ctx.gfx.drawable_size()),
            image,
            image_offset,
            text,
            text_offset,
            position:              Vec2::ZERO,
            movement:              Vec2::ZERO,
            speed:                 200.0
        }
    }

    fn screen_coordinates(&self) -> Rect {
        let mut top_left = -self.window_size * 0.5;

        if self.camera_follows_player {
            top_left += self.position;
        }

        Rect::new(top_left.x, top_left.y, self.window_size.x, self.window_size.y)
    }

    fn draw_world(&self, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(self.screen_coordinates());

        canvas.draw(&self.image, self.position - self.image_offset);
        canvas.draw(&self. text, self.position - self. text_offset);
    }

    fn draw_screen(&self, canvas: &mut Canvas) {
        canvas.set_screen_coordinates(Rect::one());

        // ui
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let fixed_movement = self.movement
            .clamp(Vec2::NEG_ONE, Vec2::ONE)
            .try_normalize()
            .unwrap_or(Vec2::ZERO);

        self.position += fixed_movement * self.speed * ctx.time.delta().as_secs_f32();

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

