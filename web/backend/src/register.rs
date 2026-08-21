// mochou-p/game/web/backend/src/register.rs

use std::collections::VecDeque;
use rspond::{MimeType, Text};
use rusqlite::{Connection, OpenFlags};
use crate::router::{ok, bad_request};


pub fn validate_body(line: String, lines: VecDeque<String>) -> Vec<u8> {
    println!("{lines:?}");

    let body       = &lines[lines.len() - 1];
    let Some(equ1) = body.find('=') else { println!("error 1");return bad_request(line); };
    let key1       = &body[..equ1];

    if key1 != "username" { println!("error 2");return bad_request(line); }

    let body       = &body[equ1+1..];
    let Some(amp1) = body.find('&') else { println!("error 3");return bad_request(line); };
    let value1     = &body[..amp1];

    let Some(username) = undo_urlencoding(value1) else {
        println!("error 4");return bad_request(line);
    };

    if username.is_empty() || username.chars().count() > 16 || !username.chars().all(|ch| is_char_allowed(ch)) {
        println!("error 5");return bad_request(line);
    }

    let body       = &body[amp1+1..];
    let Some(equ2) = body.find('=') else { println!("error 6");return bad_request(line); };
    let key2       = &body[..equ2];

    if key2 != "password" { println!("error 7");return bad_request(line); }

    let value2     = &body[equ2+1..];

    let Some(password) = undo_urlencoding(value2) else {
        println!("error 8");return bad_request(line);
    };

    if password.is_empty() || password.chars().count() > 256 {
        println!("error 9");return bad_request(line);
    }

    let worked = register(username, password);

    ok(line, MimeType::Text(Text::Html), web_frontend::register(Some(worked)))
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

fn is_char_allowed(ch: char) -> bool {
    match ch {
        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' | ' ' => true,
        _                                             => false
    }
}

fn undo_urlencoding(input: &str) -> Option<String> {
    let mut chars = input.chars();
    let mut bytes = vec![];
    
    while let Some(c) = chars.next() {
        bytes.push(match c {
            '%' => {
                let Some(a) = chars.next() else { return None; };
                let Some(b) = chars.next() else { return None; };

                let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) else {
                    return None;
                };

                byte
            },
            '+' => b' ',
            _   => c as u8
        });
    }

    String::from_utf8(bytes).ok()
}

