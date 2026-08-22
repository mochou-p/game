// mochou-p/game/web/backend/src/register.rs

use rspond::{MimeType, Text};
use rusqlite::{Connection, OpenFlags};
use crate::{router::{ok, bad_request}, utils::find_byte, Request};


pub fn validate_body(request: Request) -> Vec<u8> {
    let body       = request.body;
    let Some(equ1) = find_byte(body, b'=') else { return bad_request(); };
    let key1       = &body[..equ1];

    if key1 != b"username" { return bad_request(); }

    let body       = &body[equ1+1..];
    let Some(amp1) = find_byte(body, b'&') else { return bad_request(); };
    let value1     = &body[..amp1];

    let Some(username) = undo_urlencoding(value1) else {
        return bad_request();
    };

    if username.is_empty() || username.len() > 16 || !username.iter().all(|b| is_byte_allowed(*b)) {
        return bad_request();
    }

    let body       = &body[amp1+1..];
    let Some(equ2) = find_byte(body, b'=') else { return bad_request(); };
    let key2       = &body[..equ2];

    if key2 != b"password" { return bad_request(); }

    let value2     = &body[equ2+1..];

    let Some(password) = undo_urlencoding(value2) else {
        return bad_request();
    };

    if password.is_empty() || password.len() > 256 {
        return bad_request();
    }

    let Ok(username) = String::from_utf8(username) else {
        return bad_request();
    };
    let Ok(password) = String::from_utf8(password) else {
        return bad_request();
    };

    let worked = register(username, password);

    ok(MimeType::Text(Text::Html), web_frontend::register(Some(worked)))
}

fn register(username: String, password: String) -> bool {
    let Ok(mut conn) = Connection::open_with_flags(
        "data/db.db",
        OpenFlags::SQLITE_OPEN_READ_WRITE |
        OpenFlags::SQLITE_OPEN_CREATE     |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ) else {
        return false;
    };

    if
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA busy_timeout = 5000;
            PRAGMA foreign_keys = ON;
        ").is_err()
    {
        return false;
    }

    let Ok(tx) = conn.transaction() else { return false; };

    if 
        tx.execute(
            "INSERT INTO users (username, password) VALUES (?1, ?2)",
            (username, password)
        ).is_err()
    {
        return false;
    }

    if tx.commit().is_err() { return false; }

    true
}

fn is_byte_allowed(byte: u8) -> bool {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b' ' => true,
        _                                                     => false
    }
}

fn undo_urlencoding(input: &[u8]) -> Option<Vec<u8>> {
    let mut iter  = input.iter();
    let mut bytes = vec![];
    
    while let Some(c) = iter.next() {
        bytes.push(match c {
            b'%' => {
                let Some(a) = iter.next() else { return None; };
                let Some(b) = iter.next() else { return None; };

                let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) else {
                    return None;
                };

                byte
            },
            b'+' => b' ',
            _    => *c as u8
        });
    }

    Some(bytes)
}

