// mochou-p/game/web/backend/src/login.rs

use rspond::*;
use rusqlite::{Connection, OpenFlags, Result};
use super::{response, utils};


pub fn validate_body(body: &[u8]) -> Vec<u8> {
    let Some((username, password)) = utils::parse_signin_info(body) else {
        return response::bad_request();
    };

    let Some(token) = login(username.clone(), password) else {
        return response::internal_server_error();
    };

    response::see_other(
        format!("/users/{username}"),
        vec![
            Header::Custom(
                String::from("Set-Cookie"),
                format!("token={token}; HttpOnly; SameSite=Lax; Path=/")
            )
        ]
    )
}

// TODO: Option -> Result
fn login(username: String, password: String) -> Option<String> {
    let Ok(mut conn) = Connection::open_with_flags(
        "data/db.db",
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

