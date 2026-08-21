// mochou-p/game/web/backend/src/router.rs

use std::collections::VecDeque;
use rspond::{ResponseBuilder, HttpVersion, StatusCode, Header, Connection, MimeType, Text, Charset};
use super::{register, users};


pub fn ok(line: String, mime_type: MimeType, body: Vec<u8>) -> Vec<u8> {
    println!("\x1b[32;1m[200]\x1b[0m {line}");

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::Ok)
        .headers(vec![
            Header::Connection(Connection::Close),
            Header::ContentType(mime_type, Charset::Utf8),
            Header::ContentLength(body.len())
        ])
        .body(&body)
        .build()
}

pub fn bad_request(line: String) -> Vec<u8> {
    eprintln!("\x1b[31;1m[400]\x1b[0m {line}");

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::BadRequest)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_found(line: String) -> Vec<u8> {
    eprintln!("\x1b[33;1m[404]\x1b[0m {line}");

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::NotFound)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_implemented(line: String) -> Vec<u8> {
    eprintln!("\x1b[31;1m[501]\x1b[0m {line}");

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::NotImplemented)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn http_version_not_supported(line: String) -> Vec<u8> {
    eprintln!("\x1b[31;1m[505]\x1b[0m {line}");

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::HttpVersionNotSupported)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn handle(line: String, lines: VecDeque<String>, method: &str, path: &str) -> Vec<u8> {
    match method {
        "GET"  =>             get(line, path),
        "POST" =>            post(line, path, lines),
        _      => not_implemented(line)
    }
}

fn get(line: String, path: &str) -> Vec<u8> {
    match path {
        "/style.css" =>        ok(line, MimeType::Text(Text::Css ), web_frontend::     css(              )),
        "/"          =>        ok(line, MimeType::Text(Text::Html), web_frontend::    home(              )),
        "/users"     =>        ok(line, MimeType::Text(Text::Html), web_frontend::   users(users::query())),
        "/register"  =>        ok(line, MimeType::Text(Text::Html), web_frontend::register(None          )),
        _            => not_found(line)
    }
}

fn post(line: String, path: &str, lines: VecDeque<String>) -> Vec<u8> {
    match path {
        // NOTE: the parsing here is broken for values having unescaped '&' inside,
        //       fine for now since blablabla and password handling doesnt care
        "/register" => register::validate_body(line, lines),
        _           => not_found(line)
    }
}

