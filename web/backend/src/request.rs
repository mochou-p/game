// mochou-p/game/web/backend/src/request.rs

use std::collections::HashMap;
use super::utils;


pub const MAX_LEN: usize = 2048;

pub struct Request<'a> {
    pub method:  &'a [u8],
    pub path:    &'a [u8],
    pub version: &'a [u8],
    pub headers: HashMap<&'a [u8], &'a [u8]>,
    pub body:    &'a [u8]
}

impl<'a> Request<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        let     window           = data;

        let     end          = utils::find_byte(window, b' ')?;
        let     method       = &window[..end];
        let     window       = &window[end+1..];

        let     end          = utils::find_byte(window, b' ')?;
        let     path         = &window[..end];
        let     window       = &window[end+1..];

        let     end          = utils::find_bytes(window, b"\r\n")?;
        let     version      = &window[..end];
        let     window       = &window[end+2..];

        let     body_start   = utils::find_bytes(window, b"\r\n\r\n")?;

        let mut header_range = &window[..body_start+2];
        let     body         = &window[body_start+4..];
        let mut headers      = HashMap::new();

        while let Some(i) = utils::find_bytes(header_range, b"\r\n") {
            let line  = &header_range[..i];

            let j     = utils::find_bytes(line, b": ")?;
            let key   = &line[..j];
            let value = &line[j+2..];

            if headers.insert(key, value).is_some() {
                return None;
            }

            header_range = &header_range[i+2..];
        }

        Some(Self { method, path, version, headers, body })
    }
}
