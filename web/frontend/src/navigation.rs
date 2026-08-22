// mochou-p/game/web/frontend/src/navigation.rs

use webuild::Tag;


#[derive(Copy, Clone, PartialEq)]
pub enum Nav {
    Home,
    Users,
    Register
}

impl Nav {
    fn href(self) -> &'static str {
        match self {
            Self::Home     => "/",
            Self::Users    => "/users",
            Self::Register => "/register"
        }
    }

    fn inner_text(self) -> &'static str {
        match self {
            Self::Home     => "home",
            Self::Users    => "users",
            Self::Register => "register"
        }
    }
}

pub fn navigation(current: Nav) -> Tag {
    Tag::new("header").children(&[
        Tag::new("nav").children(&[
            Tag::new("div").children(&[
                link(current, Nav::Home),
                link(current, Nav::Users)
            ]),
            Tag::new("div").children(&[
                link(current, Nav::Register)
            ])
        ])
    ])
}

fn link(current: Nav, nav: Nav) -> Tag {
    let attributes = if nav == current {
        &[("href", nav.href()), ("id", "nav-current")] as &[(&str, &str)]
    } else {
        &[("href", nav.href())] as &[(&str, &str)]
    };

    Tag::new("a")
        .attributes(attributes)
        .children(&[
            Tag::new("p").children(&[nav.inner_text()])
        ])
}

