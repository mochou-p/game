// mochou-p/game/game/client/src/network.rs

use std::time::Duration;
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::watch::Receiver as Watch;


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

async fn actor(
        n2g_w:   Sender<ServerMessage>,
    mut g2n_r: Receiver<ClientMessage>,
    mut stop:     Watch<bool>
) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let     tcp_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::tcp::PORT);
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

    let     udp_client_address = format!("{}.{}.{}.{}:0",  ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
    let     udp_server_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::udp::PORT);
    let mut udp                = match UdpSocket::bind(udp_client_address).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to bind UDP socket: {err} (giving up on networking :D)");
            return;
        }
    };

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

    utils::ok!("network actor online");

    let mut tcp_read_buffer  = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];
    let mut tcp_write_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut udp_read_buffer  = [0; game_protocol::udp::MAX_SERVER_LEN as usize];
    let mut udp_write_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];

    loop {
        tokio::select! {
            // TODO: its not cancel safe T_T
            result = game_protocol::tcp::recv_s2c(&mut tcp, &mut tcp_read_buffer) => {
                let Some(incoming) = result else {
                    continue;
                };

                utils::debug!("INCOMING TCP: {incoming:?}");

                if let Err(err) = n2g_w.send(ServerMessage::Tcp(incoming)).await {
                    utils::error!("n2g channel send failed: {err}");
                    continue;
                }
            },

            result = game_protocol::udp::recv_s2c(&mut udp, &mut udp_read_buffer) => {
                let Some(incoming) = result else {
                    continue;
                };

                utils::debug!("INCOMING UDP: {incoming:?}");

                if let Err(err) = n2g_w.send(ServerMessage::Udp(incoming)).await {
                    utils::error!("n2g channel send failed: {err}");
                    continue;
                }
            },

            result = g2n_r.recv() => {
                let message = match result {
                    Some(some) => some,
                    None => {
                        utils::error!("g2n channel closed");
                        break;
                    }
                };

                match message {
                    ClientMessage::Tcp(outgoing) => {
                        utils::debug!("OUTGOING TCP: {outgoing:?}");

                        game_protocol::tcp::send_c2s(
                            &mut tcp,
                            outgoing,
                            &mut tcp_write_buffer
                        ).await;
                    },
                    ClientMessage::Udp(outgoing) => {
                        utils::debug!("OUTGOING UDP: {outgoing:?}");

                        game_protocol::udp::send_c2s(
                            &mut udp,
                            outgoing,
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

    utils::info!("network actor offline");
}

