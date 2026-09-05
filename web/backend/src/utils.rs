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

pub fn undo_urlencoding(input: &[u8]) -> Option<Vec<u8>> {
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

