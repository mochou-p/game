// mochou-p/game/game/server/src/tcp.rs

use tokio::net::TcpListener;
use tokio::sync::watch::Receiver;


pub async fn bind(mut stop: Receiver<bool>) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];
    const PORT:    u16     = 10079;

    let  address = format!("{}.{}.{}.{}:{PORT}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
    let _server  = TcpListener::bind(address).await.unwrap();

    println!("\x1b[32;1m[{} TCP online]\x1b[0m", env!("CARGO_BIN_NAME"));

    loop {
        stop.changed().await.unwrap();
        if *stop.borrow() {
            break;
        }
    }

    println!("\x1b[31;1m[{} TCP offline]\x1b[0m", env!("CARGO_BIN_NAME"));
}

