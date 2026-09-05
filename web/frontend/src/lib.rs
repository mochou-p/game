// mochou-p/game/web/frontend/src/lib.rs

pub mod assets;
    mod navigation;
    mod home;
    mod users;
    mod signin;
    mod register;
    mod login;
    mod user;

use webuild::{HtmlBuilder, Tag};


pub enum Page {
    Home,
    Users(Vec<String>),
    Register,
    Login,
    User(String)
}

pub fn render(loginee: Option<String>, page: Page) -> Vec<u8> {
    match page {
        Page::Home           =>     home::render(loginee          ),
        Page::Users(users)   =>    users::render(loginee, users   ),
        Page::Register       => register::render(loginee          ),
        Page::Login          =>    login::render(loginee          ),
        Page::User(username) =>     user::render(loginee, username)
    }
}

fn base(title: &str, children: &[Tag]) -> Vec<u8> {
    HtmlBuilder::default()
        .lang("en")
        .charset("UTF-8")
        .responsive_viewport()
        .title(&format!("{title} | game"))
        .favicon("image/x-icon", "/assets/icons/favicon.ico")
        .css("/assets/stylesheets/main.css")
        .body_children(children)
        .build()
        .as_bytes()
        .to_vec()
}

fn content(children: &[Tag]) -> Tag {
    Tag::new("main").children(children)
}

fn content_centered(children: &[Tag]) -> Tag {
    Tag::new("main").attributes(&[("class", "centered")]).children(children)
}

