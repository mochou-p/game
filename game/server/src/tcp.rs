// mochou-p/game/game/server/src/tcp.rs

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio::sync::watch::Receiver;
use game_core::PlayerId;
use super::client::Client;
use super::state::State;


pub async fn bind(
        state: Arc<State>,
    mut stop:  Receiver<bool>
) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::tcp::PORT);
    let server  = match TcpListener::bind(address).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to bind listener: {err}");
            return;
        }
    };

    utils::ok!("online");

    loop {
        tokio::select! {
            result = server.accept() => {
                match result {
                    Ok((stream, address)) => {
                        tokio::spawn(handle_client(stream, address, state.clone()));
                    },
                    Err(err) => {
                        utils::warning!("failed to accept a peer: {err}");
                    }
                }
            },

            _ = stop.changed() => {
                if *stop.borrow() {
                    utils::debug!("saw ^C");
                    break;
                }
            }
        }
    }

    utils::info!("offline");
}

async fn handle_client(
    mut stream:  TcpStream,
        address: SocketAddr,
        state:   Arc<State>
) {
    utils::ok!("{address}: connected");

    let mut  read_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];

    let (chan_w, mut chan_r) = mpsc::channel(32);
    let mut token_maybe      = None;

    loop {
        // TODO: cancel safe?
        tokio::select! {
            result = game_protocol::tcp::recv_c2s(&mut stream, address, &mut read_buffer) => {
                let Some(incoming) = result else {
                    utils::debug!("{address}: dropping client");
                    break;
                };

                utils::debug!("{address}: INCOMING: {incoming:?}");

                if let Some(outgoing) = {
                    match incoming {
                        game_protocol::tcp::ClientToServer::LetsShakeHands => {
                            if token_maybe.is_some() {
                                utils::warning!("{address}: asking for a token again");
                                continue;
                            }

                            let Some(rng) = utils::csprng() else {
                                break;
                            };

                            let token = game_protocol::Token(rng);

                            {
                                let mut cs = state.clients.write().await;

                                cs.token2client.insert(
                                    token,
                                    Arc::new(RwLock::new(Client::new(chan_w.clone())))
                                );
                            }

                            token_maybe = Some(token);
                            Some(game_protocol::tcp::ServerToClient::Handshake { token })
                        }
                    }
                } {
                    utils::debug!("{address}: OUTGOING: {outgoing:?}");

                    if !game_protocol::tcp::send_s2c(
                        &mut stream,
                        address,
                        outgoing,
                        &mut write_buffer
                    ).await {
                        utils::debug!("{address}: dropping client");
                        break;
                    }
                }
            },

            result = chan_r.recv() => {
                let outgoing = match result {
                    Some(some) => some,
                    None       => {
                        utils::error!("client channel closed");
                        break;
                    }
                };

                utils::debug!("{address}: OUTGOING: {outgoing:?}");

                if !game_protocol::tcp::send_s2c(
                    &mut stream,
                    address,
                    outgoing,
                    &mut write_buffer
                ).await {
                    utils::debug!("{address}: dropping client");
                    break;
                }
            }
        }
    }

    if
        let Some(token) = token_maybe
        &&
        let Some(id   ) = clean_up_client(state.clone(), token).await
    {
        // TODO: broadcast, and im probably holding cs lock for too long
        //       and i also hate these reads
        let others = {
            let     cs       = state.clients.read().await;
            let mut channels = Vec::with_capacity(cs.token2client.0.len());

            for client in cs.token2client.0.values() {
                let c = client.read().await;
                channels.push(c.tcp.clone());
            }

            channels
        };

        for other_channel in others {
            let _ = other_channel.send(
                game_protocol::tcp::ServerToClient::PlayerLeft { id }
            ).await;
        }
    }

    utils::info!("{address}: disconnected");
}

async fn clean_up_client(
    state: Arc<State>,
    token: game_protocol::Token
) -> Option<PlayerId> {
    let mut cs = state.clients.write().await;

    let client = cs.token2client.remove(&token)?;

    let (player_id_maybe, udp_maybe) = {
        let c = client.read().await;
        (c.player, c.udp)
    };

    if let Some(player_id) = player_id_maybe {
        let mut ps = state.players.write().await;
        ps.remove(&player_id);
    }

    if let Some(udp) = udp_maybe {
        cs.udpaddr2token.remove(&udp);
    }

    player_id_maybe
}

