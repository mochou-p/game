// mochou-p/game/game/client/src/network.rs

use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::watch::Receiver;


pub fn spawn(stop: Receiver<bool>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(actor(stop))
    })
}

async fn actor(mut stop: Receiver<bool>) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let  tcp_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::TCP_PORT);
    let _tcp         = TcpStream::connect(tcp_address).await.unwrap();

    let udp_client_address = format!("{}.{}.{}.{}:0",  ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
    let udp_server_address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::UDP_PORT);
    let udp                = UdpSocket::bind(udp_client_address).await.unwrap();

    udp.connect(udp_server_address).await.unwrap();

    println!("\x1b[32;1m[{} network actor online]\x1b[0m", env!("CARGO_BIN_NAME"));

    loop {
        stop.changed().await.unwrap();
        if *stop.borrow() {
            break;
        }
    }

    println!("\x1b[31;1m[{} network actor offline]\x1b[0m", env!("CARGO_BIN_NAME"));
}

