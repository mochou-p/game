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
    let user = match database_core::user_from_session_token(token) {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to get user from session token: {err}");
            None
        }
    };

    match path {
        b"/assets/stylesheets/main.css" => {
            match web_frontend::assets::css() {
                Ok(ok) => {
                    response::ok(
                        MediaType::Text(Text::Css, Charset::Utf8),
                        ok
                    )
                },
                Err(err) => {
                    utils::error!("failed to load css: {err}");
                    response::internal_server_error()
                }
            }
        },
        b"/assets/icons/favicon.ico" => {
            match web_frontend::assets::favicon() {
                Ok(ok) => {
                    response::ok(
                        MediaType::Image(Image::Icon),
                        ok
                    )
                },
                Err(err) => {
                    utils::error!("failed to load favicon: {err}");
                    response::internal_server_error()
                }
            }
        },
        b"/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    user,
                    web_frontend::Page::Home
                )
            )
        },
        b"/users" | b"/users/" => {
            match database_core::all_users() {
                Ok(ok) => {
                    response::ok(
                        MediaType::Text(Text::Html, Charset::Utf8),
                        web_frontend::render(
                            user,
                            web_frontend::Page::Users(ok)
                        )
                    )
                },
                Err(err) => {
                    utils::error!("failed to get all users: {err}");
                    response::internal_server_error()
                }
            }
        },
        b"/register" | b"/register/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    user,
                    web_frontend::Page::Register
                )
            )
        },
        b"/login" | b"/login/" => {
            response::ok(
                MediaType::Text(Text::Html, Charset::Utf8),
                web_frontend::render(
                    user,
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
                            user,
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

