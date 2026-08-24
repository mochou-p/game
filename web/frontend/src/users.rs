// mochou-p/game/web/frontend/src/users.rs

use webuild::Tag;
use crate::{base, content, navigation::{Nav, navigation}};


pub fn render(loginee: Option<String>, users: Vec<String>) -> Vec<u8> {
    let mut rows = Vec::with_capacity(1 + users.len());

    rows.push(
        Tag::new("tr").children(&[
            Tag::new("th").children(&["username"])
        ])
    );

    for name in users {
        rows.push(
            Tag::new("tr").children(&[
                Tag::new("td").children(&[
                    Tag::new("a").attributes(&[("href", &format!("/users/{name}"))]).children(&[&name])
                ])
            ])
        );
    }

    base("users", &[
        navigation(loginee, Nav::Users),
        Tag::new("hr"),
        content(&[Tag::new("table").children(&rows)])
    ])
}

