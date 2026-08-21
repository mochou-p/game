// mochou-p/game/web/backend/src/users.rs

use rusqlite::{Connection, OpenFlags, Error};


pub fn query() -> Vec<(i64, String, String, i64)> {
    let conn = Connection::open_with_flags(
        "data/db.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ).unwrap();

    conn.execute_batch("PRAGMA busy_timeout = 5000;").unwrap();

    let mut stmt = conn.prepare("SELECT id, username, password, created_at FROM users").unwrap();

    stmt.query_map([], |row| {
        Ok((
            row.get(0).unwrap(),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
            row.get(3).unwrap()
        ))
    })
        .unwrap()
        .collect::<Result<Vec<_>, Error>>()
        .unwrap()
}

