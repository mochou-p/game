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
            Tag::new("div").attributes(&[("class", "nav-group")]).children(
                &links(current, [Nav::Home, Nav::Users])
            ),
            Tag::new("div").attributes(&[("class", "nav-group")]).children(
                &links(current, [Nav::Register])
            )
        ])
    ])
}

fn links<const N: usize>(current: Nav, navs: [Nav; N]) -> [Tag; N] {
    navs.map(|nav| {
        let attributes = if nav == current {
            &[("href", nav.href()), ("class", "nav-current")] as &[(&str, &str)]
        } else {
            &[("href", nav.href())] as &[(&str, &str)]
        };

        Tag::new("a")
            .attributes(attributes)
            .children(&[
                Tag::new("h3").children(&[nav.inner_text()])
            ])
    })
}

