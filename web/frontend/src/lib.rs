// mochou-p/game/web/frontend/src/lib.rs

mod navigation;

use webuild::Tag;
use navigation::{navigation, Nav};


pub fn css() -> Vec<u8> {
    webuild::macros::css!(
        * {
            box-sizing: "border-box";
            border:     "none";
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
            display:        "flex";
            flex-direction: "column";
            width:          "100%";
            max-width:      "800px";
            height:         "100%";
            background:     "#800f2f";
            color:          "#ffccd5";
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

        main {
            display:         "flex";
            flex:            "1";
            justify-content: "center";
            align-items:     "center";
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

        input {
            background: "#fff0f3";
            color:      "#800f2f";
        }

        form > button {
            background: "#ff4d6d";
            color:      "#fff0f3";
            padding:    "0 8px 0 8px";
        }

        form > button:hover {
            cursor: "pointer";
        }

        table {
            background:      "#a4133c";
            color:           "#ffb3c1";
            border-collapse: "collapse";
        }

        th, td {
            border:  "1px solid #590d22";
            padding: "0 8px 0 8px";
        }

        th {
            color: "#fff0f3";
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

pub fn users(users: Vec<(i64, String, String, i64)>) -> Vec<u8> {
    let mut rows = Vec::with_capacity(users.len() + 1);

    rows.push(
        Tag::new("tr").children(&[
            Tag::new("th").children(&["id"      ]),
            Tag::new("th").children(&["username"]),
            Tag::new("th").children(&["password"]),
            Tag::new("th").children(&["joined"  ])
        ])
    );

    for user in users {
        rows.push(
            Tag::new("tr").children(&[
                Tag::new("td").children(&[user.0.to_string()]),
                Tag::new("td").children(&[user.1            ]),
                Tag::new("td").children(&[user.2            ]),
                Tag::new("td").children(&[user.3.to_string()])
            ])
        );
    }

    base("users | game", &[
        navigation(Nav::Users),
        content(&[Tag::new("table").children(&rows)])
    ])
}

pub fn register(registered: Option<bool>) -> Vec<u8> {
    let children = if let Some(worked) = registered {
        &[
            Tag::new("p").children(&[
                if worked {
                    "registered successfully!"
                } else {
                    "database error: boo hoo bozo"
                }
            ])
        ]
    } else {
        &[
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
                Tag::new("button").attributes(&[("type", "submit")]).children(&["register"])
            ])
        ]
    };

    base("registration | game", &[
        navigation(Nav::Register),
        content(children)
    ])
}

fn content(children: &[Tag]) -> Tag {
    Tag::new("main").children(children)
}

