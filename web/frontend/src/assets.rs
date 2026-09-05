// mochou-p/game/web/frontend/src/assets.rs

macro_rules!     css_filepath { () => { "web/frontend/assets/stylesheets/main.css" }; }
macro_rules! favicon_filepath { () => { "web/frontend/assets/icons/favicon.ico"    }; }

pub fn css() -> Vec<u8> {
    #[cfg(debug_assertions)] {
        std::fs::read(css_filepath!()).unwrap()
    }

    #[cfg(not(debug_assertions))] {
        include_bytes!(concat!("../../../", css_filepath!())).to_vec()
    }
}

pub fn favicon() -> Vec<u8> {
    #[cfg(debug_assertions)] {
        std::fs::read(favicon_filepath!()).unwrap()
    }

    #[cfg(not(debug_assertions))] {
        include_bytes!(concat!("../../../", favicon_filepath!())).to_vec()
    }
}

