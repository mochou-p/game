// mochou-p/game/game/server/src/tcp.rs

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio::sync::watch::Receiver;
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

    if let Some(token) = token_maybe {
        let mut cs = state.clients.write().await;

        if let Some(client) = cs.token2client.remove(&token) {
            let c = client.read().await;

            if let Some(udp) = c.udp {
                cs.udpaddr2token.remove(&udp);
            }
        }
    }

    utils::info!("{address}: disconnected");
}

