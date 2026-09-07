// mochou-p/game/game/server/src/tcp.rs

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::watch::Receiver;
use game_protocol::postcard;


pub async fn bind(mut stop: Receiver<bool>) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::tcp::PORT);
    let server  = TcpListener::bind(address).await.unwrap();

    println!("\x1b[32;1m[{} TCP online]\x1b[0m", env!("CARGO_BIN_NAME"));

    loop {
        tokio::select! {
            result = server.accept() => {
                let (client, _) = result.unwrap();
                tokio::spawn(handle_client(client));
            },

            _ = stop.changed() => {
                if *stop.borrow() {
                    break;
                }
            }
        }
    }

    println!("\x1b[31;1m[{} TCP offline]\x1b[0m", env!("CARGO_BIN_NAME"));
}

async fn handle_client(mut client: TcpStream) {
    let mut  read_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];

    loop {
        let length = client.read_u16().await.unwrap();
        assert!(length <= game_protocol::tcp::MAX_CLIENT_LEN);
        let length = length as usize;

        client.read_exact(&mut read_buffer[..length]).await.unwrap();
        let message = postcard::from_bytes(&read_buffer[..length]).unwrap();

        match message {
            game_protocol::tcp::ClientToServer::LetsShakeHands => {
                let token   = 1234567890;
                let message = game_protocol::tcp::ServerToClient::Handshake { token };
                let bytes   = postcard::to_slice(&message, &mut write_buffer).unwrap();
                let length  = bytes.len();
                assert!(length <= game_protocol::tcp::MAX_SERVER_LEN as usize);

                client.write_u16(length as u16).await.unwrap();
                client.write_all(bytes        ).await.unwrap();
            }
        }
    }
}

