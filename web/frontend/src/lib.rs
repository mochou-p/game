// mochou-p/game/web/frontend/src/lib.rs

mod navigation;

use webuild::Tag;
use navigation::{navigation, Nav};


pub fn css() -> Vec<u8> {
    webuild::macros::css!(
        * {
            box-sizing: "border-box";
            margin:     "0";
            padding:    "0";
        }

        html {
            display:         "flex";
            justify-content: "center";
            width:           "100vw";
            height:          "100vh";
            background:      "#590d22";
        }

        body {
            width:      "100%";
            max-width:  "800px";
            height:     "100%";
            background: "#800f2f";
            color:      "#ffccd5";
        }

        a {
            text-decoration: "none";
            color:           "inherit";
        }

        nav {
            display:         "flex";
            justify-content: "space-between";
            margin-bottom:   "2px";
            background:      "#a4133c";
            color:           "#ffb3c1";
        }

        .nav-group {
            display: "flex";
        }

        .nav-group > a {
            padding: "0 16px 0 16px";
        }

        .nav-group > a:hover {
            color: "#fff0f3";
        }

        .nav-current {
            font-weight: "bold";
            background:  "#ff4d6d";
            color:       "#fff0f3";
        }
    )
        .as_bytes()
        .to_vec()
}

fn base(title: &str, children: &[Tag]) -> Vec<u8> {
    webuild::DocumentBuilder::default()
        .with_lang("en")
        .with_charset("UTF-8")
        .with_responsive_viewport(true)
        .with_title(title)
        .with_css("style.css")
        .with_body_children(children)
        .build_html()
        .as_bytes()
        .to_vec()
}

pub fn home() -> Vec<u8> {
    base("home | game", &[
        navigation(Nav::Home),
        content(&[
            Tag::new("p").children(&["welcome :D"])
        ])
    ])
}

pub fn users() -> Vec<u8> {
    base("users | game", &[
        navigation(Nav::Users),
        content(&[
            Tag::new("p").children(&["noone :c"])
        ])
    ])
}

pub fn register() -> Vec<u8> {
    base("registration | game", &[
        navigation(Nav::Register),
        content(&[
            Tag::new("p").children(&["wip!"])
        ])
    ])
}

fn content(children: &[Tag]) -> Tag {
    Tag::new("main").children(children)
}

