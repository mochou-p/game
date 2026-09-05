// mochou-p/game/web/frontend/src/login.rs

use super::{signin, navigation::Nav};


pub fn render(loginee: Option<String>) -> Vec<u8> {
    signin::render("login", loginee, Nav::Login, "login")
}

