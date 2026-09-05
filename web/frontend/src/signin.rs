// mochou-p/game/web/frontend/src/signin.rs

use webuild::Tag;
use super::{base, content_centered, navigation::{Nav, navigation}};


pub fn render(title: &str, loginee: Option<String>, nav: Nav, button: &str) -> Vec<u8> {
    let username_regex = "[A-Za-z0-9_ ]{1,16}";
    let password_regex = "^.{1,256}";

    base(title, &[
        navigation(loginee, nav),
        Tag::new("hr"),
        content_centered(&[
            Tag::new("form").attributes(&[("method", "POST")]).children(&[
                Tag::new("label").attributes(&[("for", "username")]).children(&[
                    format!("username {username_regex}").as_str()
                ]),
                Tag::new("br"),
                Tag::new("input").attributes(&[
                    ("type",        "text"),
                    ("maxlength",   "16"),
                    ("pattern",     username_regex),
                    ("name",        "username"),
                    ("placeholder", "DookieFartPoopyShart")
                ]),
                Tag::new("br"),
                Tag::new("label").attributes(&[("for", "password")]).children(&[
                    format!("password {password_regex}").as_str()
                ]),
                Tag::new("br"),
                Tag::new("input").attributes(&[
                    ("type",        "password"),
                    ("maxlength",   "256"),
                    ("pattern",     password_regex),
                    ("name",        "password"),
                    ("placeholder", "six seven")
                ]),
                Tag::new("br"),
                Tag::new("br"),
                Tag::new("button").attributes(&[("type", "submit")]).children(&[button])
            ])
        ])
    ])
}

