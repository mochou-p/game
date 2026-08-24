// mochou-p/game/web/backend/src/router.rs

use rspond::*;
use web_frontend::*;
use super::{response::*, register, login, logout, users, Request};


pub fn handle(request: Request) -> Vec<u8> {
    match request.method {
        b"GET"  =>  get(request.path, request.headers.get(b"Cookie" as &[u8])),
        b"POST" => post(request.path, request.body),
        _       => not_implemented()
    }
}

fn get(path: &[u8], token: Option<&&[u8]>) -> Vec<u8> {
    match path {
        b"/style.css" =>        ok(MediaType::Text(Text::Css,  Charset::Utf8),    css()),
        b"/"          =>        ok(MediaType::Text(Text::Html, Charset::Utf8), render(users::from(token), Page::Home                 )),
        b"/users"     =>        ok(MediaType::Text(Text::Html, Charset::Utf8), render(users::from(token), Page::Users(users::query()))),
        b"/register"  =>        ok(MediaType::Text(Text::Html, Charset::Utf8), render(users::from(token), Page::Register             )),
        b"/login"     =>        ok(MediaType::Text(Text::Html, Charset::Utf8), render(users::from(token), Page::Login                )),
        _             => not_found()
    }
}

fn post(path: &[u8], body: &[u8]) -> Vec<u8> {
    match path {
        b"/register" => register::validate_body(body),
        b"/login"    =>    login::validate_body(body),
        b"/logout"   =>   logout::remove_cookie(),
        _            => not_found()
    }
}

