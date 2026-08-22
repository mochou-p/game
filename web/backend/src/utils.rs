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

