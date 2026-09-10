// mochou-p/game/game/server/src/udp.rs

use tokio::net::UdpSocket;
use tokio::sync::watch::Receiver;


pub async fn bind(mut stop: Receiver<bool>) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let     address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::udp::PORT);
    let mut server  = match UdpSocket::bind(address).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to bind socket: {err}");
            return;
        }
    };

    let mut  read_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::udp::MAX_SERVER_LEN as usize];

    utils::ok!("online");

    loop {
        tokio::select! {
            result = game_protocol::udp::recv_c2s(&mut server, &mut read_buffer) => {
                let Some((address, incoming)) = result else {
                    continue;
                };

                utils::debug!("{address}: INCOMING: {incoming:?}");

                let outgoing = match incoming {
                    game_protocol::udp::ClientToServer::Temp => {
                        game_protocol::udp::ServerToClient::Temp
                    }
                };

                utils::debug!("{address}: OUTGOING: {outgoing:?}");

                if !game_protocol::udp::send_s2c(
                    &mut server,
                    address,
                    outgoing,
                    &mut write_buffer
                ).await {
                    continue;
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

