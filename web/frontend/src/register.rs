// mochou-p/game/web/frontend/src/register.rs

use super::{signin, navigation::Nav};


pub fn render(loginee: Option<String>) -> Vec<u8> {
    signin::render("register", loginee, Nav::Register, "register")
}

