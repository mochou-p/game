// mochou-p/game/game/server/src/udp.rs

use tokio::net::UdpSocket;
use tokio::sync::watch::Receiver;
use game_protocol::postcard;


pub async fn bind(mut stop: Receiver<bool>) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::udp::PORT);
    let server  = UdpSocket::bind(address).await.unwrap();

    let mut  read_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::udp::MAX_SERVER_LEN as usize];

    println!("\x1b[32;1m[{} UDP online]\x1b[0m", env!("CARGO_BIN_NAME"));

    loop {
        tokio::select! {
            result = server.recv_from(&mut read_buffer) => {
                let (length, address) = result.unwrap();
                assert!(length <= game_protocol::udp::MAX_CLIENT_LEN as usize);

                let message = postcard::from_bytes(&read_buffer[..length]).unwrap();

                match message {
                    game_protocol::udp::ClientToServer::Temp => {
                        let message = game_protocol::udp::ServerToClient::Temp;
                        let bytes   = postcard::to_slice(&message, &mut write_buffer).unwrap();
                        let length  = bytes.len();
                        assert!(length <= game_protocol::udp::MAX_SERVER_LEN as usize);

                        server.send_to(bytes, address).await.unwrap();
                    }
                }
            },

            _ = stop.changed() => {
                if *stop.borrow() {
                    break;
                }
            }
        }
    }

    println!("\x1b[31;1m[{} UDP offline]\x1b[0m", env!("CARGO_BIN_NAME"));
}

