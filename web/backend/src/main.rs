// mochou-p/game/web/backend/src/main.rs

mod request;
mod response;
mod router;
mod register;
mod login;
mod logout;
mod users;
mod validation;
mod utils;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};
use request::Request;


#[tokio::main]
async fn main() {
    database_core::setup();

    const ADDRESS: [u8; 4] = [127, 0, 0, 1];
    const PORT:    u16     = 10069;

    let address = format!("{}.{}.{}.{}:{PORT}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
    let server  = TcpListener::bind(address).await.unwrap();

    println!("\x1b[32;1m[{} online]\x1b[0m", env!("CARGO_BIN_NAME"));

    loop {
        let (client, _) = server.accept().await.unwrap();
        tokio::spawn(async move { handle_client(client).await; });
    }
}

async fn handle_client(mut client: TcpStream) {
    let (buffer, count) =   read_request(&mut client).await;
    let response        = handle_request(&buffer[..count]);

    client.write_all(&response).await.unwrap();
}

async fn read_request(client: &mut TcpStream) -> ([u8; request::MAX_LEN], usize) {
    let mut buffer = [0; request::MAX_LEN];
    let     count  = client.read(&mut buffer).await.unwrap();

    assert!(count < request::MAX_LEN);

    (buffer, count)
}

fn handle_request(data: &[u8]) -> Vec<u8> {
    let Some(request) = Request::parse(data) else {
        return response::bad_request();
    };

    if request.version != b"HTTP/1.1" {
        return response::http_version_not_supported();
    }

    router::handle(request)
}

