// mochou-p/game/web/frontend/src/style.rs

pub fn css() -> Vec<u8> {
    webuild::macros::css!(
        // layout --------------------------------

        * {
            box-sizing:     "border-box";
            border:         "none";
            margin:         "0";
            padding:        "0";
            padding-inline: "0";
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

        h3 {
            font-weight: "normal";
        }

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

        #nav-current > h3 {
            font-weight: "900";
        }

        nav > div > form > button {
            font-family: "inherit";
            font:        "inherit";
            background:  "inherit";
            color:       "inherit";
        }

        nav > div > form > button:hover {
            cursor:          "pointer";
            text-decoration: "underline";
        }
    )
        .as_bytes()
        .to_vec()
}
