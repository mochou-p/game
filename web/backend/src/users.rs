// mochou-p/game/web/backend/src/users.rs

pub fn username(value: &[u8]) -> Option<String> {
    let username = String::from_utf8(value.to_vec()).ok()?;

    // TODO: query info

    Some(username)
}

