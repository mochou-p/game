// mochou-p/game/database/core/src/lib.rs

use rusqlite::{Connection, OpenFlags, OptionalExtension, Result, Error};


const DB_FILEPATH: &str = "database/game.db";

pub fn setup() {
    let mut conn = Connection::open_with_flags(
        DB_FILEPATH,
        OpenFlags::SQLITE_OPEN_READ_WRITE |
        OpenFlags::SQLITE_OPEN_CREATE     |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ).unwrap();

    conn.execute_batch("
        PRAGMA journal_mode = WAL;
        PRAGMA busy_timeout = 5000;
        PRAGMA foreign_keys = ON;
    ").unwrap();

    let tx = conn.transaction().unwrap();

    tx.execute_batch("
        CREATE TABLE IF NOT EXISTS users (
            id        INTEGER PRIMARY KEY AUTOINCREMENT,
            username  TEXT NOT NULL UNIQUE,
            password  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sessions (
            token    TEXT PRIMARY KEY,
            user_id  INTEGER NOT NULL,

            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        );
    ").unwrap();

    tx.commit().unwrap();
}

// TODO: Option -> Result
pub fn login(username: String, password: String) -> Option<String> {
    let Ok(mut conn) = Connection::open_with_flags(
        DB_FILEPATH,
        OpenFlags::SQLITE_OPEN_READ_ONLY |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ) else {
        return None;
    };

    if conn.execute_batch("PRAGMA busy_timeout = 5000;").is_err() {
        return None;
    }

    let Ok(tx) = conn.transaction() else {
        return None;
    };

    let Ok(Some(real_password)): Result<Option<String>> = tx.query_row(
        "SELECT password FROM users WHERE username = ?1",
        (username.clone(),),
        |row| row.get(0)
    ) else {
        return None;
    };

    if tx.commit().is_err() {
        return None;
    }

    if password == real_password {
        let token = username;
        Some(token)
    } else {
        None
    }
}

// TODO: Option -> Result
pub fn register(username: String, password: String) -> Option<String> {
    let token = username.clone();

    let Ok(mut conn) = Connection::open_with_flags(
        DB_FILEPATH,
        OpenFlags::SQLITE_OPEN_READ_WRITE |
        OpenFlags::SQLITE_OPEN_CREATE     |
        OpenFlags::SQLITE_OPEN_NO_MUTEX
    ) else {
        return None;
    };

    if
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA busy_timeout = 5000;
            PRAGMA foreign_keys = ON;
        ").is_err()
    {
        return None;
    }

    let Ok(tx) = conn.transaction() else {
        return None;
    };

    let Ok(Some(user_id)): Result<Option<i64>> = tx.query_row(
        "INSERT INTO users (username, password) VALUES (?1, ?2) RETURNING id",
        (username, password),
        |row| row.get(0)
    ) else {
        return None;
    };

    if tx.execute(
        "INSERT INTO sessions (token, user_id) VALUES (?1, ?2)",
        (token.clone(), user_id)
    ).is_err() {
        return None;
    }

    if tx.commit().is_err() {
        return None;
    }

    Some(token)
}

pub fn all_users() -> Vec<String> {
    let conn = Connection::open_with_flags(
        DB_FILEPATH,
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

pub fn user_from_session_token(value: Option<&&[u8]>) -> Option<String> {
    let token = value?;

    // TODO: temp while cookie is `token=...`
    let token = &token[6..];

    let token = String::from_utf8((*token).to_vec()).ok()?;

    let mut conn = Connection::open_with_flags(
        DB_FILEPATH,
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

