// mochou-p/game/web/backend/src/validation.rs

use super::utils;


// TODO: Option -> Result
pub fn parse_signin_info(body: &[u8]) -> Option<(String, String)> {
    let equ1 = utils::find_byte(body, b'=')?;
    let key1 = &body[..equ1];

    if key1 != b"username" { return None; }

    let body     = &body[equ1+1..];
    let amp1     = utils::find_byte(body, b'&')?;
    let value1   = &body[..amp1];
    let username = utils::undo_urlencoding(value1)?;

    // [A-Za-z0-9_ ]{1,16}
    if username.is_empty() || username.len() > 16 || !username.iter().all(|b| is_byte_allowed_in_username(*b)) {
        return None;
    }

    let body = &body[amp1+1..];
    let equ2 = utils::find_byte(body, b'=')?;
    let key2 = &body[..equ2];

    if key2 != b"password" { return None; }

    let value2   = &body[equ2+1..];
    let password = utils::undo_urlencoding(value2)?;

    // ^.{1,256}
    if password.is_empty() || password.len() > 256 {
        return None;
    }

    let username = String::from_utf8(username).ok()?;
    let password = String::from_utf8(password).ok()?;

    Some((username, password))
}

fn is_byte_allowed_in_username(byte: u8) -> bool {
    matches!(byte, b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b' ')
}

