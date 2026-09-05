// mochou-p/game/web/frontend/src/user.rs

use webuild::Tag;
use super::{base, content, navigation::{Nav, navigation}};


pub fn render(loginee: Option<String>, user: String) -> Vec<u8> {
    base(&format!("{user} - profile"), &[
        navigation(loginee, Nav::User(user.clone())),
        Tag::new("hr"),
        content(&[Tag::new("h2").children(&[user.as_str()])])
    ])
}

