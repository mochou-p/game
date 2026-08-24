// mochou-p/game/web/backend/src/main.rs

mod request;
mod response;
mod router;
mod register;
mod login;
mod logout;
mod users;
mod utils;

use rusqlite::{Connection, OpenFlags};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};
use request::Request;


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

    tx.execute_batch("
        CREATE TABLE IF NOT EXISTS users (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            username  TEXT NOT NULL UNIQUE,
            password  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sessions (
            token    TEXT PRIMARY KEY,
            user_id  INTEGER NOT NULL,

            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
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

    println!("\x1b[32;1m[online]\x1b[0m");

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

