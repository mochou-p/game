// mochou-p/game/web/backend/src/main.rs

mod router;
mod register;
mod users;

use std::collections::VecDeque;
use rusqlite::{Connection, OpenFlags};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream};


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
    let request  =   read_request(&mut client).await;
    let response = handle_request(&request   );

    client.write_all(&response).await.unwrap();
}

async fn read_request(client: &mut TcpStream) -> String {
    const LEN: usize = 2048;

    let mut buffer = [0; LEN];
    let     count  = client.read(&mut buffer).await.unwrap();

    assert!(count < LEN);

    String::from_utf8_lossy(&buffer[..count]).into_owned()
}

fn handle_request(request: &str) -> Vec<u8> {
    let mut lines = request.lines().map(str::to_owned).collect::<VecDeque<String>>();

    let Some(line) = lines.pop_front() else {
        return router::bad_request(String::new());
    };

    let mut parts = line.split_whitespace().map(str::to_owned).collect::<VecDeque<String>>();

    let Some(method ) = parts.pop_front() else { return router::bad_request(line); };
    let Some(path   ) = parts.pop_front() else { return router::bad_request(line); };
    let Some(version) = parts.pop_front() else { return router::bad_request(line); };

    if version != "HTTP/1.1" { return router::http_version_not_supported(line); }

    router::handle(line, lines, &method, &path)
}

