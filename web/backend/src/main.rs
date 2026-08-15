// mochou-p/game/web/backend/src/main.rs

mod router;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
async fn main() {
    const ADDRESS: &str = "127.0.0.1";
    const PORT:    u16  = 10069;

    let server = TcpListener::bind(format!("{ADDRESS}:{PORT}")).await.unwrap();

    println!("\x1b[32;1m> online\x1b[0m");

    loop {
        let (client, _) = server.accept().await.unwrap();
        tokio::spawn(async move { handle_client(client).await; });
    }
}

async fn handle_client(mut client: TcpStream) {
    let request  =   read_request(&mut client).await;
    let response = handle_request(&request   );

    client.write_all(&response).await.unwrap();
}

async fn read_request(client: &mut TcpStream) -> String {
    const LEN: usize = 1024;

    let mut buffer = [0; LEN];
    let     count  = client.read(&mut buffer).await.unwrap();

    assert!(count < LEN);

    String::from_utf8_lossy(&buffer[..count]).into_owned()
}

fn handle_request(request: &str) -> Vec<u8> {
    let     line  = request.lines().next().unwrap().to_owned();
    let mut parts = line.split_whitespace().map(str::to_owned).collect::<std::collections::VecDeque<String>>();

    let Some(method ) = parts.pop_front() else { return router::bad_request(line); };
    let Some(path   ) = parts.pop_front() else { return router::bad_request(line); };
    let Some(version) = parts.pop_front() else { return router::bad_request(line); };

    if version != "HTTP/1.1" { return router::http_version_not_supported(line); }

    router::handle(line, &method, &path)
}

