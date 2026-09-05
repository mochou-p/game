// mochou-p/game/web/frontend/src/home.rs

use webuild::Tag;
use super::{base, content_centered, navigation::{Nav, navigation}};


pub fn render(loginee: Option<String>) -> Vec<u8> {
    base("home", &[
        navigation(loginee, Nav::Home),
        Tag::new("hr"),
        content_centered(&[
            Tag::new("p").children(&["welcome :D"])
        ])
    ])
}

