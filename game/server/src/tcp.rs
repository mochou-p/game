// mochou-p/game/game/server/src/tcp.rs

use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch::Receiver;


static COUNTER: AtomicU32 = AtomicU32::new(0);

pub async fn bind(mut stop: Receiver<bool>) {
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
                        tokio::spawn(handle_client(stream, address));
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

async fn handle_client(mut stream: TcpStream, address: SocketAddr) {
    utils::ok!("{address}: connected");

    let mut  read_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];

    loop {
        let Some(incoming) = game_protocol::tcp::recv_c2s(
            &mut stream,
            address,
            &mut read_buffer
        ).await else {
            utils::debug!("{address}: dropping connection due to error");
            break;
        };

        utils::debug!("{address}: INCOMING: {incoming:?}");

        let outgoing = match incoming {
            game_protocol::tcp::ClientToServer::LetsShakeHands => {
                let token = COUNTER.fetch_add(1, Ordering::Relaxed);

                game_protocol::tcp::ServerToClient::Handshake { token }
            }
        };

        utils::debug!("{address}: OUTGOING: {outgoing:?}");

        if !game_protocol::tcp::send_s2c(
            &mut stream,
            address,
            outgoing,
            &mut write_buffer
        ).await {
            utils::debug!("{address}: dropping connection due to error");
            break;
        }
    }

    utils::info!("{address}: disconnected");
}

