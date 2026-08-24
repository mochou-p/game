// mochou-p/game/web/frontend/src/lib.rs

mod style;
mod navigation;
mod home;
mod users;
mod register;
mod login;

use webuild::{DocumentBuilder, Tag};

pub use style::css;


pub enum Page {
    Home,
    Users(Vec<String>),
    Register,
    Login
}

pub fn render(loginee: Option<String>, page: Page) -> Vec<u8> {
    match page {
        Page::Home         =>     home::render(loginee       ),
        Page::Users(users) =>    users::render(loginee, users),
        Page::Register     => register::render(loginee       ),
        Page::Login        =>    login::render(loginee       )
    }
}

fn base(title: &str, children: &[Tag]) -> Vec<u8> {
    DocumentBuilder::default()
        .with_lang("en")
        .with_charset("UTF-8")
        .with_responsive_viewport(true)
        .with_title(format!("{title} | game"))
        .with_css("style.css")
        .with_body_children(children)
        .build_html()
        .as_bytes()
        .to_vec()
}

fn content(children: &[Tag]) -> Tag {
    Tag::new("main").children(children)
}

