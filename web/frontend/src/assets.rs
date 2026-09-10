// mochou-p/game/web/frontend/src/assets.rs

macro_rules!     css_filepath { () => { "web/frontend/assets/stylesheets/main.css" }; }
macro_rules! favicon_filepath { () => { "web/frontend/assets/icons/favicon.ico"    }; }

pub fn css() -> std::io::Result<Vec<u8>> {
    #[cfg(debug_assertions)] {
        std::fs::read(css_filepath!())
    }

    #[cfg(not(debug_assertions))] {
        Ok(include_bytes!(concat!("../../../", css_filepath!())).to_vec())
    }
}

pub fn favicon() -> std::io::Result<Vec<u8>> {
    #[cfg(debug_assertions)] {
        std::fs::read(favicon_filepath!())
    }

    #[cfg(not(debug_assertions))] {
        Ok(include_bytes!(concat!("../../../", favicon_filepath!())).to_vec())
    }
}

