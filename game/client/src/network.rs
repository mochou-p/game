// mochou-p/game/game/client/src/network.rs

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::watch::Receiver as Watch;
use game_protocol::postcard;


pub enum ServerMessage {
    Tcp(game_protocol::tcp::ServerToClient),
    Udp(game_protocol::udp::ServerToClient)
}

pub enum ClientMessage {
    Tcp(game_protocol::tcp::ClientToServer),
    Udp(game_protocol::udp::ClientToServer)
}

pub fn spawn(
    n2g_w:   Sender<ServerMessage>,
    g2n_r: Receiver<ClientMessage>,
    stop:     Watch<bool>
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(actor(n2g_w, g2n_r, stop))
    })
}

async fn actor(
        n2g_w:   Sender<ServerMessage>,
    mut g2n_r: Receiver<ClientMessage>,
    mut stop:     Watch<bool>
) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let     tcp_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::tcp::PORT);
    let mut tcp         = TcpStream::connect(tcp_address).await.unwrap();

    let udp_client_address = format!("{}.{}.{}.{}:0",  ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
    let udp_server_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::udp::PORT);
    let udp                = UdpSocket::bind(udp_client_address).await.unwrap();

    udp.connect(udp_server_address).await.unwrap();

    println!("\x1b[32;1m[{} network actor online]\x1b[0m", env!("CARGO_BIN_NAME"));

    let mut tcp_read_buffer  = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];
    let mut tcp_write_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut udp_read_buffer  = [0; game_protocol::udp::MAX_SERVER_LEN as usize];
    let mut udp_write_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];

    let message = game_protocol::tcp::ClientToServer::LetsShakeHands;
    let   bytes = postcard::to_slice(&message, &mut tcp_write_buffer).unwrap();

    tcp.write_u16(bytes.len() as u16).await.unwrap();
    tcp.write_all(bytes             ).await.unwrap();

    loop {
        tokio::select! {
            // TODO: its not cancel safe T_T
            result = tcp.read_u16() => {
                let length = result.unwrap();
                assert!(length <= game_protocol::tcp::MAX_SERVER_LEN);
                let length = length as usize;

                // NOTE: does this await block the select?
                tcp.read_exact(&mut tcp_read_buffer[..length]).await.unwrap();
                let message = postcard::from_bytes(&tcp_read_buffer[..length]).unwrap();

                n2g_w.send(ServerMessage::Tcp(message)).await.unwrap();
            },

            result = udp.recv(&mut udp_read_buffer) => {
                let length = result.unwrap();
                assert!(length <= game_protocol::udp::MAX_SERVER_LEN as usize);

                let message = postcard::from_bytes(&udp_read_buffer[..length]).unwrap();

                n2g_w.send(ServerMessage::Udp(message)).await.unwrap();
            },

            result = g2n_r.recv() => {
                let message = result.unwrap();

                match message {
                    ClientMessage::Tcp(tcp_message) => {
                        let bytes  = postcard::to_slice(&tcp_message, &mut tcp_write_buffer).unwrap();
                        let length = bytes.len();
                        assert!(length <= game_protocol::tcp::MAX_CLIENT_LEN as usize);

                        tcp.write_u16(length as u16).await.unwrap();
                        tcp.write_all(bytes        ).await.unwrap();
                    },
                    ClientMessage::Udp(udp_message) => {
                        let bytes  = postcard::to_slice(&udp_message, &mut udp_write_buffer).unwrap();
                        let length = bytes.len();
                        assert!(length <= game_protocol::udp::MAX_CLIENT_LEN as usize);

                        udp.send(bytes).await.unwrap();
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

    println!("\x1b[31;1m[{} network actor offline]\x1b[0m", env!("CARGO_BIN_NAME"));
}

