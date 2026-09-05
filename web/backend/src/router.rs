// mochou-p/game/web/backend/src/router.rs

use rspond::*;
use super::{response, register, login, logout, users, Request};


pub fn handle(request: Request) -> Vec<u8> {
    match request.method {
        b"GET"  =>  get(request.path, request.headers.get(b"Cookie" as &[u8])),
        b"POST" => post(request.path, request.body),
        _       => response::not_implemented()
    }
}

fn get(path: &[u8], token: Option<&&[u8]>) -> Vec<u8> {
    match path {
        b"/assets/stylesheets/main.css" => {
            response::ok(
                MediaType::Text(Text::Css, Charset::Utf8),
                web_frontend::assets::css()
            )
        },
        b"/assets/icons/favicon.ico" => {
            response::ok(
                MediaType::Image(Image::Icon),
                web_frontend::assets::favicon()
            )
        },
        b"/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    database_core::user_from_session_token(token),
                    web_frontend::Page::Home
                )
            )
        },
        b"/users" | b"/users/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    database_core::user_from_session_token(token),
                    web_frontend::Page::Users(database_core::all_users())
                )
            )
        },
        b"/register" | b"/register/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    database_core::user_from_session_token(token),
                    web_frontend::Page::Register
                )
            )
        },
        b"/login" | b"/login/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    database_core::user_from_session_token(token),
                    web_frontend::Page::Login
                )
            )
        },
        _ => {
            if path.starts_with(b"/users/") {
                let username = &path[7..];

                if let Some(user_info) = users::username(username) {
                    return response::ok(
                        MediaType::Text(Text::Html, Charset::Utf8),
                        web_frontend::render(
                            database_core::user_from_session_token(token),
                            web_frontend::Page::User(user_info)
                        )
                    );
                }
            }

            response::not_found()
        }
    }
}

fn post(path: &[u8], body: &[u8]) -> Vec<u8> {
    match path {
        b"/register" => register::validate_body(body),
        b"/login"    =>    login::validate_body(body),
        b"/logout"   =>   logout::remove_cookie(),
        _            => response::not_found()
    }
}

