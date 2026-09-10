// mochou-p/game/database/core/src/lib.rs

use rusqlite::{Connection, OpenFlags, OptionalExtension, Statement, Transaction, Result, Error};


const DB_FILEPATH: &str = "database/game.db";

pub fn setup() -> Result<()> {
    Database::writable()?
        .transaction(|tx| {
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
            ")
        })
}

pub fn login(username: String, password: String) -> Result<Option<String>> {
    let real_password = Database::readonly()?
        .transaction(|tx| {
            tx.query_row(
                "SELECT password FROM users WHERE username = ?1",
                (username.clone(),),
                |row| row.get(0)
            ).optional()
        })?;

    Ok(
        if Some(password) == real_password {
            let token = username;
            Some(token)
        } else {
            None
        }
    )
}

pub fn register(username: String, password: String) -> Result<Option<String>> {
    let token = username.clone();

    Database::writable()?
        .transaction(|tx| {
            let user_id: i64 = tx.query_row(
                "INSERT INTO users (username, password) VALUES (?1, ?2) RETURNING id",
                (username, password),
                |row| row.get(0)
            )?;

            tx.execute(
                "INSERT INTO sessions (token, user_id) VALUES (?1, ?2)",
                (token.clone(), user_id)
            )
        })?;

    Ok(Some(token))
}

pub fn all_users() -> Result<Vec<String>> {
    Database::readonly()?
        .prepare("SELECT username FROM users", |stmt| {
            stmt.query_map([], |row| Ok(row.get(0)?))?
                .collect::<Result<Vec<_>, Error>>()
        })
}

pub fn user_from_session_token(value: Option<&&[u8]>) -> Result<Option<String>> {
    let Some(token) = value else {
        return Ok(None);
    };

    // TODO: temp while cookie is `token=...`
    let token = &token[6..];

    let Some(token) = String::from_utf8((*token).to_vec()).ok() else {
        utils::error!("could not utf8 decode token: {token:?}");
        return Ok(None);
    };

    Database::readonly()?
        .transaction(|tx| {
            let user_id: Option<String> = tx.query_row(
                "SELECT user_id FROM sessions WHERE token = ?1",
                (token,),
                |row| row.get(0)
            ).optional()?;

            let Some(user_id) = user_id else {
                return Ok(None);
            };

            let username = tx.query_row(
                "SELECT username FROM users WHERE id = ?1",
                (user_id,),
                |row| row.get(0)
            )?;

            Ok(Some(username))
        })
}

struct Database {
    conn: Connection
}

impl Database {
    fn readonly() -> Result<Self> {
        let conn = Connection::open_with_flags(
            DB_FILEPATH,
            OpenFlags::SQLITE_OPEN_READ_ONLY
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
        )?;

        conn.execute_batch("PRAGMA busy_timeout = 5000;")?;

        Ok(Self { conn })
    }

    fn writable() -> Result<Self> {
        let conn = Connection::open_with_flags(
            DB_FILEPATH,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX
        )?;

        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA busy_timeout = 5000;
            PRAGMA foreign_keys = ON;
        ")?;

        Ok(Self { conn })
    }

    fn transaction<T>(&mut self, f: impl FnOnce(&Transaction) -> Result<T>) -> Result<T> {
        let tx = self.conn.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;

        Ok(result)
    }

    fn prepare<T>(&mut self, statement: &str, f: impl FnOnce(&mut Statement) -> Result<T>) -> Result<T> {
        let mut stmt = self.conn.prepare(statement)?;
        f(&mut stmt)
    }
}

