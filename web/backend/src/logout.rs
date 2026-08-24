// mochou-p/game/web/backend/src/logout.rs

use rspond::*;
use super::response;


pub fn remove_cookie() -> Vec<u8> {
    response::see_other(
        String::from("/"),
        vec![
            Header::Custom(
                String::from("Set-Cookie"),
                String::from("token=; Max-Age=0; HttpOnly; SameSite=Lax; Path=/")
            )
        ]
    )
}

