// mochou-p/game/web/backend/src/main.rs

mod request;
mod response;
mod router;
mod register;
mod login;
mod logout;
mod users;
mod validation;
mod web_utils;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};
use request::Request;


#[tokio::main]
async fn main() {
    if let Err(err) = database_core::setup() {
        utils::error!("failed to setup database: {err}");
        return;
    }

    const ADDRESS: [u8; 4] = [127, 0, 0, 1];
    const PORT:    u16     = 10069;

    let address = format!("{}.{}.{}.{}:{PORT}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3]);
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
                    Ok((stream, _)) => {
                        tokio::spawn(handle_client(stream));
                    },
                    Err(err) => {
                        utils::warning!("failed to accept a peer: {err}");
                    }
                }
            },

            _ = tokio::signal::ctrl_c() => {
                println!();
                utils::debug!("saw ^C");
                break;
            }
        }
    }

    utils::info!("offline");
}

async fn handle_client(mut stream: TcpStream) {
    let Some((buffer, count)) = read_request(&mut stream).await else {
        return;
    };

    let response = handle_request(&buffer[..count]);

    if let Err(err) = stream.write_all(&response).await {
        utils::warning!("failed to write bytes: {err}");
        return;
    }
}

async fn read_request(stream: &mut TcpStream) -> Option<([u8; request::MAX_LEN], usize)> {
    let mut buffer = [0; request::MAX_LEN];

    let count = match stream.read(&mut buffer).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::warning!("failed to read request: {err}");
            return None;
        }
    };

    // NOTE: even `=` because i dont want to read unbounded
    if count == 0 || count >= request::MAX_LEN {
        utils::warning!("invalid request length: {count}");
        return None;
    }

    Some((buffer, count))
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

