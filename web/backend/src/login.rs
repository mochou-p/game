// mochou-p/game/web/backend/src/login.rs

use super::{response, validation};


pub fn validate_body(body: &[u8]) -> Vec<u8> {
    let Some((username, password)) = validation::parse_signin_info(body) else {
        return response::bad_request();
    };

    let Some(token) = database_core::login(username.clone(), password) else {
        return response::internal_server_error();
    };

    response::see_other(
        format!("/users/{username}"),
        vec![
            rspond::Header::Custom(
                String::from("Set-Cookie"),
                format!("token={token}; HttpOnly; SameSite=Lax; Path=/")
            )
        ]
    )
}

