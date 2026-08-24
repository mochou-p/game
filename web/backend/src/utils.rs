// mochou-p/game/web/backend/src/utils.rs

pub fn find_byte(bytes: &[u8], pattern: u8) -> Option<usize> {
    for (i, byte) in bytes.iter().enumerate() {
        if *byte == pattern {
            return Some(i);
        }
    }

    None
}

pub fn find_bytes(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    for (i, window) in bytes.windows(pattern.len()).enumerate() {
        if window == pattern {
            return Some(i);
        }
    }

    None
}

// TODO: Option -> Result
pub fn parse_signin_info(body: &[u8]) -> Option<(String, String)> {
    let equ1       = find_byte(body, b'=')?;
    let key1       = &body[..equ1];

    if key1 != b"username" { return None; }

    let body       = &body[equ1+1..];
    let amp1       = find_byte(body, b'&')?;
    let value1     = &body[..amp1];

    let username   = undo_urlencoding(value1)?;

    if username.is_empty() || username.len() > 16 || !username.iter().all(|b| is_byte_allowed_in_username(*b)) {
        return None;
    }

    let body       = &body[amp1+1..];
    let equ2       = find_byte(body, b'=')?;
    let key2       = &body[..equ2];

    if key2 != b"password" { return None; }

    let value2     = &body[equ2+1..];

    let password   = undo_urlencoding(value2)?;

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

fn undo_urlencoding(input: &[u8]) -> Option<Vec<u8>> {
    let mut iter  = input.iter();
    let mut bytes = vec![];

    while let Some(c) = iter.next() {
        bytes.push(match c {
            b'%' => {
                let a = iter.next()?;
                let b = iter.next()?;

                let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) else {
                    return None;
                };

                byte
            },
            b'+' => b' ',
            _    => *c
        });
    }

    Some(bytes)
}

