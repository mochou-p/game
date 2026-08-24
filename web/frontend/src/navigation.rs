// mochou-p/game/web/frontend/src/navigation.rs

use webuild::Tag;


const NAV_HEADING: &str = "h3";

#[derive(PartialEq)]
pub enum Nav {
    Home,
    Users,
    Register,
    Login,
    User(String),
    Logout
}

impl Nav {
    fn render(&self, current: &Self) -> Tag {
        match self {
            Self::Home           => link(self, current,          "/",                  "home"    ),
            Self::Users          => link(self, current,          "/users",             "users"   ),
            Self::Register       => link(self, current,          "/register",          "register"),
            Self::Login          => link(self, current,          "/login",             "login"   ),
            Self::User(username) => link(self, current, &format!("/users/{username}"), username  ),
            Self::Logout         => {
                Tag::new("form").attributes(&[("method", "POST"), ("action", "/logout")]).children(&[
                    Tag::new("button").attributes(&[("type", "submit")]).children(&[
                        Tag::new(NAV_HEADING).children(&["logout"])
                    ])
                ])
            }
        }
    }
}

pub fn navigation(loginee: Option<String>, current: Nav) -> Tag {
    Tag::new("header").children(&[
        Tag::new("nav").children(&[
            Tag::new("div").children(&[
                Nav::Home .render(&current),
                Nav::Users.render(&current)
            ]),
            Tag::new("div").children(
                &if let Some(username) = loginee {
                    [
                        Nav::User(username).render(&current),
                        Nav::Logout        .render(&current)
                    ]
                } else {
                    [
                        Nav::Register.render(&current),
                        Nav::Login   .render(&current)
                    ]
                }
            )
        ])
    ])
}

fn link(nav: &Nav, current: &Nav, href: &str, text: &str) -> Tag {
    let attributes = if *nav == *current {
        &[("href", href), ("id", "nav-current")] as &[(&str, &str)]
    } else {
        &[("href", href)] as &[(&str, &str)]
    };

    Tag::new("a")
        .attributes(attributes)
        .children(&[
            Tag::new(NAV_HEADING).children(&[text])
        ])
}

