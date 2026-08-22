// mochou-p/game/web/backend/src/main.rs

mod router;
mod register;
mod users;
mod utils;

use std::collections::HashMap;
use rusqlite::{Connection, OpenFlags};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};
use utils::{find_byte, find_bytes};


const BUFFER_LEN: usize = 2048;

fn setup_db() {
    let mut conn = Connection::open_with_flags(
        "data/db.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE |
        OpenFlags::SQLITE_OPEN_CREATE     |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ).unwrap();

    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA busy_timeout = 5000;
        PRAGMA foreign_keys = ON;
    ").unwrap();

    let tx = conn.transaction().unwrap();

    // NOTE: batch because i will have more tables of course
    tx.execute_batch("
        CREATE TABLE IF NOT EXISTS users (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            username   TEXT NOT NULL UNIQUE,
            password   TEXT NOT NULL,
            created_at INTEGER DEFAULT (unixepoch())
        );
    ").unwrap();

    tx.commit().unwrap();
}

#[tokio::main]
async fn main() {
    setup_db();

    const ADDRESS: &str = "127.0.0.1";
    const PORT:    u16  = 10069;

    let server = TcpListener::bind(format!("{ADDRESS}:{PORT}")).await.unwrap();

    println!("\x1b[32;1m> online\x1b[0m\n");

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

async fn read_request(client: &mut TcpStream) -> ([u8; BUFFER_LEN], usize) {
    let mut buffer = [0; BUFFER_LEN];
    let     count  = client.read(&mut buffer).await.unwrap();

    assert!(count < BUFFER_LEN);

    (buffer, count)
}

fn handle_request(data: &[u8]) -> Vec<u8> {
    let Some(request) = Request::parse(data) else {
        return router::bad_request();
    };

    if request.version != b"HTTP/1.1" {
        return router::http_version_not_supported();
    }

    router::handle(request)
}

struct Request<'a> {
    method:  &'a [u8],
    path:    &'a [u8],
    version: &'a [u8],
    headers: HashMap<&'a [u8], &'a [u8]>,
    body:    &'a [u8]
}

impl<'a> Request<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        let window = data;

        let Some(end) = find_byte(window, b' ') else {
            return None;
        };
        let method = &window[..end];
        let window = &window[end+1..];

        let Some(end) = find_byte(window, b' ') else {
            return None;
        };
        let path   = &window[..end];
        let window = &window[end+1..];

        let Some(end) = find_bytes(window, b"\r\n") else {
            return None;
        };
        let version = &window[..end];
        let window  = &window[end+2..];

        let Some(body_start) = find_bytes(window, b"\r\n\r\n") else {
            return None;
        };

        let mut header_range = &window[..body_start+2];
        let     body         = &window[body_start+4..];
        let mut headers      = HashMap::new();

        while let Some(i) = find_bytes(header_range, b"\r\n") {
            let line = &header_range[..i];

            let Some(j) = find_bytes(line, b": ") else {
                return None;
            };
            let key   = &line[..j];
            let value = &line[j+2..];

            if headers.insert(key, value).is_some() {
                return None;
            }

            header_range = &header_range[i+2..];
        }

        Some(Self { method, path, version, headers, body })
    }
}

