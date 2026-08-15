// mochou-p/game/web/backend/src/router.rs

use rspond::prelude::*;


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

pub fn handle(line: String, method: &str, path: &str) -> Vec<u8> {
    if method != "GET" { return not_implemented(line); }

    get(line, path)
}

fn get(line: String, path: &str) -> Vec<u8> {
    if path == "/" {
        home(line)
    } else {
        not_found(line)
    }
}

fn home(line: String) -> Vec<u8> {
    println!("\x1b[32;1m[200]\x1b[0m {line}");

    let body = b"<!DOCTYPE html><html><body><h1>hi world</h1></body></html>";

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::Ok)
        .headers(vec![
            Header::Connection(Connection::Close),
            Header::ContentType(MimeType::Html, Charset::Utf8),
            Header::ContentLength(body.len())
        ])
        .body(body)
        .build()
}

