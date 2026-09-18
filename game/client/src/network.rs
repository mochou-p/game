// mochou-p/game/game/client/src/network.rs

use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::watch::Receiver as Watch;


const IP_ENVVAR: &str = "GAME_SERVER_IP";

#[derive(Debug)]
pub enum ServerMessage {
    Tcp(game_protocol::tcp::ServerToClient),
    Udp(game_protocol::udp::ServerToClient)
}

#[derive(Debug)]
pub enum ClientMessage {
    Tcp(game_protocol::tcp::ClientToServer),
    Udp(game_protocol::udp::ClientToServer)
}

pub fn spawn(
    n2g_w:   Sender<ServerMessage>,
    g2n_r: Receiver<ClientMessage>,
    stop:     Watch<bool>
) -> Option<std::thread::JoinHandle<()>> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => {
            Some(std::thread::spawn(move || {
                runtime.block_on(actor(n2g_w, g2n_r, stop))
            }))
        },
        Err(err) => {
            utils::error!("failed to create tokio runtime: {err}");
            None
        }
    }
}

fn client_address() -> String {
    if std::env::var(IP_ENVVAR).is_ok() {
        utils::info!("making a public client");
        format!("0.0.0.0:0")
    } else {
        utils::info!("making a localhost client");
        format!("127.0.0.1:0")
    }
}

fn server_address(port: u16) -> String {
    let address = {
        if let Ok(ip) = std::env::var(IP_ENVVAR) {
            utils::info!("connecting to a remote server from ${IP_ENVVAR}");
            format!("{ip}:{port}")
        } else {
            utils::info!("connecting to localhost (empty ${IP_ENVVAR})");
            format!("127.0.0.1:{port}")
        }
    };

    utils::important!("\"{address}\"");
    address
}

async fn actor(
        n2g_w:   Sender<ServerMessage>,
    mut g2n_r: Receiver<ClientMessage>,
    mut stop:     Watch<bool>
) {
    let     tcp_address = server_address(game_protocol::tcp::PORT);
    let mut tcp         = loop {
        match TcpStream::connect(&tcp_address).await {
            Ok (ok ) => break ok,
            Err(err) => {
                let timeout = 5;
                utils::warning!("failed to connect to TCP server: {err} (retrying after {timeout} seconds)");

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_secs(timeout)) => {
                        continue;
                    }

                    _ = stop.changed() => {
                        if *stop.borrow() {
                            utils::debug!("saw game end");
                            return;
                        }
                    }
                }
            }
        }
    };

    let     udp_client_address = client_address();
    let mut udp                = match UdpSocket::bind(udp_client_address).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to bind UDP socket: {err} (giving up on networking :D)");
            return;
        }
    };

    let udp_server_address = server_address(game_protocol::udp::PORT);
    while let Err(err) = udp.connect(&udp_server_address).await {
        let timeout = 5;
        utils::warning!("failed to connect to UDP server: {err} (retrying after {timeout} seconds)");

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(timeout)) => {
                continue;
            }

            _ = stop.changed() => {
                if *stop.borrow() {
                    utils::debug!("saw game end");
                    return;
                }
            }
        }
    }

    super::game::ONLINE.store(true, Ordering::Relaxed);
    utils::ok!("online");

    let mut tcp_read_buffer  = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];
    let mut tcp_write_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut udp_read_buffer  = [0; game_protocol::udp::MAX_SERVER_LEN as usize];
    let mut udp_write_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];

    loop {
        // TODO: cancel safe?
        tokio::select! {
            result = game_protocol::tcp::recv_s2c(&mut tcp, &mut tcp_read_buffer) => {
                let Some(incoming) = result else {
                    break;
                };

                utils::debug!("INCOMING TCP: {incoming:?}");

                if let Err(err) = n2g_w.send(ServerMessage::Tcp(incoming)).await {
                    utils::error!("n2g channel send failed: {err}");
                    continue;
                }
            },

            result = game_protocol::udp::recv_s2c(&mut udp, &mut udp_read_buffer) => {
                let Some(incoming) = result else {
                    break;
                };

                utils::debug!("INCOMING UDP: {incoming:?}");

                if let Err(err) = n2g_w.send(ServerMessage::Udp(incoming)).await {
                    utils::error!("n2g channel send failed: {err}");
                    continue;
                }
            },

            result = g2n_r.recv() => {
                let outgoing = match result {
                    Some(some) => some,
                    None => {
                        utils::error!("g2n channel closed");
                        break;
                    }
                };

                match outgoing {
                    ClientMessage::Tcp(outgoing_tcp) => {
                        utils::debug!("OUTGOING TCP: {outgoing_tcp:?}");

                        game_protocol::tcp::send_c2s(
                            &mut tcp,
                            outgoing_tcp,
                            &mut tcp_write_buffer
                        ).await;
                    },
                    ClientMessage::Udp(outgoing_udp) => {
                        utils::debug!("OUTGOING UDP: {outgoing_udp:?}");

                        game_protocol::udp::send_c2s(
                            &mut udp,
                            outgoing_udp,
                            &mut udp_write_buffer
                        ).await;
                    }
                }
            },

            _ = stop.changed() => {
                if *stop.borrow() {
                    utils::debug!("saw game end");
                    break;
                }
            }
        }
    }

    super::game::ONLINE.store(false, Ordering::Relaxed);
    utils::info!("offline");
}

