// mochou-p/game/web/backend/src/router.rs

use rspond::{ResponseBuilder, HttpVersion, StatusCode, Header, Connection, MimeType, Text, Charset};
use super::{register, users, Request};


pub fn ok(mime_type: MimeType, body: Vec<u8>) -> Vec<u8> {
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

pub fn bad_request() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::BadRequest)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_found() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::NotFound)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_implemented() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::NotImplemented)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn http_version_not_supported() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::HttpVersionNotSupported)
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn handle(request: Request) -> Vec<u8> {
    match request.method {
        b"GET"  =>  get(request),
        b"POST" => post(request),
        _       => not_implemented()
    }
}

fn get(request: Request) -> Vec<u8> {
    match request.path {
        b"/style.css" =>        ok(MimeType::Text(Text::Css ), web_frontend::     css(              )),
        b"/"          =>        ok(MimeType::Text(Text::Html), web_frontend::    home(              )),
        b"/users"     =>        ok(MimeType::Text(Text::Html), web_frontend::   users(users::query())),
        b"/register"  =>        ok(MimeType::Text(Text::Html), web_frontend::register(None          )),
        _             => not_found()
    }
}

fn post(request: Request) -> Vec<u8> {
    match request.path {
        // NOTE: the parsing here is broken for values having unescaped '&' inside,
        //       fine for now since blablabla and password handling doesnt care
        b"/register" => register::validate_body(request),
        _            => not_found()
    }
}

