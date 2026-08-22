// mochou-p/game/web/frontend/src/lib.rs

mod navigation;

use webuild::Tag;
use navigation::{navigation, Nav};


pub fn css() -> Vec<u8> {
    webuild::macros::css!(
        // layout --------------------------------

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
        }

        body {
            display:        "flex";
            flex-direction: "column";
            width:          "100%";
            max-width:      "800px";
            height:         "100%";
        }

        nav {
            display:         "flex";
            justify-content: "space-between";
        }

        main {
            display:         "flex";
            flex:            "1";
            justify-content: "center";
            align-items:     "center";
        }

        nav > div {
            display: "flex";
            gap:     "24px";
        }

        // style ---------------------------------

        a {
            text-decoration: "none";
            color:           "inherit";
        }

        a:hover {
            text-decoration: "underline";
        }

        table {
            border-collapse: "collapse";
        }

        th, td {
            border:  "1px solid #fff";
            padding: "0 8px 0 8px";
        }

        html {
            background: "#000";
        }

        body {
            color: "#fff";
        }

        hr {
            border: "1px solid #fff";
        }

        #nav-current {
            font-weight: "900";
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
        .with_title(format!("{title} | game"))
        .with_css("style.css")
        .with_body_children(children)
        .build_html()
        .as_bytes()
        .to_vec()
}

pub fn home() -> Vec<u8> {
    base("home", &[
        navigation(Nav::Home),
        Tag::new("hr"),
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

    base("users", &[
        navigation(Nav::Users),
        Tag::new("hr"),
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

    base("register", &[
        navigation(Nav::Register),
        Tag::new("hr"),
        content(children)
    ])
}

fn content(children: &[Tag]) -> Tag {
    Tag::new("main").children(children)
}

