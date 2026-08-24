// mochou-p/game/web/frontend/src/login.rs

use webuild::Tag;
use crate::{base, content, navigation::{Nav, navigation}};


pub fn render(loginee: Option<String>) -> Vec<u8> {
    base("login", &[
        navigation(loginee, Nav::Login),
        Tag::new("hr"),
        content(&[
            Tag::new("form").attributes(&[("method", "POST")]).children(&[
                Tag::new("label").attributes(&[("for", "username")]).children(&["username"]),
                Tag::new("br"),
                Tag::new("input").attributes(&[
                    ("type",        "text"),
                    ("maxlength",   "16"),
                    ("pattern",     "[A-Za-z0-9_ ]{1,16}"),
                    ("name",        "username"),
                    ("placeholder", "DookieFartPoopyShart")
                ]),
                Tag::new("br"),
                Tag::new("label").attributes(&[("for", "password")]).children(&["password"]),
                Tag::new("br"),
                Tag::new("input").attributes(&[
                    ("type",        "password"),
                    ("maxlength",   "256"),
                    ("pattern",     "^.{1,256}"),
                    ("name",        "password"),
                    ("placeholder", "six seven")
                ]),
                Tag::new("br"),
                Tag::new("br"),
                Tag::new("button").attributes(&[("type", "submit")]).children(&["login"])
            ])
        ])
    ])
}

