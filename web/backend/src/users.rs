// mochou-p/game/web/backend/src/users.rs

use rusqlite::{Connection, OpenFlags, Error, OptionalExtension};


pub fn query() -> Vec<String> {
    let conn = Connection::open_with_flags(
        "data/db.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ).unwrap();

    conn.execute_batch("PRAGMA busy_timeout = 5000;").unwrap();

    let mut stmt = conn.prepare("SELECT username FROM users").unwrap();

    stmt.query_map([], |row| Ok(row.get(0).unwrap()))
        .unwrap()
        .collect::<Result<Vec<_>, Error>>()
        .unwrap()
}

pub fn from(token: Option<&&[u8]>) -> Option<String> {
    let token = token?;

    // TODO: temp while cookie is `token=...`
    let token = &token[6..];

    let token = String::from_utf8((*token).to_vec()).ok()?;

    let mut conn = Connection::open_with_flags(
        "data/db.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ).unwrap();

    conn.execute_batch("PRAGMA busy_timeout = 5000;").unwrap();

    let tx = conn.transaction().unwrap();

    let user_id: i64 = tx.query_row(
        "SELECT user_id FROM sessions WHERE token = ?1",
        (token,),
        |row| row.get(0)
    )
        .optional()
        .ok()??;

    let username = tx.query_row(
        "SELECT username FROM users WHERE id = ?1",
        (user_id,),
        |row| row.get(0)
    )
        .unwrap();

    tx.commit().unwrap();

    Some(username)
}

