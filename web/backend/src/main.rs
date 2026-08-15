// mochou-p/game/web/backend/src/main.rs

mod router;

use std::io::{Read as _, Write as _};
use std::net::{TcpListener, TcpStream};


fn main() {
    const ADDRESS: &str = "127.0.0.1";
    const PORT:    u16  = 10069;

    let server = TcpListener::bind(format!("{ADDRESS}:{PORT}")).unwrap();

    println!("\x1b[32;1m> online\x1b[0m");

    for client in server.incoming() {
        handle_client(client.unwrap());
    }
}

fn handle_client(mut client: TcpStream) {
    let request  =   read_request(&mut client);
    let response = handle_request(&request   );

    client.write_all(&response).unwrap();
}

fn read_request(client: &mut TcpStream) -> String {
    const LEN: usize = 1024;

    let mut buffer = [0; LEN];
    let     count  = client.read(&mut buffer).unwrap();

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

