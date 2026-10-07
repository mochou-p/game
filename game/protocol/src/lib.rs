// mochou-p/game/game/protocol/src/lib.rs

pub mod tcp;
pub mod udp;

use serde::{Serialize, Deserialize};


const IP_ENVVAR: &str = "GAME_SERVER_IP";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token(pub [u8; 32]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Nonce(pub [u8; 16]);

pub fn client_address() -> String {
    if std::env::var(IP_ENVVAR).is_ok() {
        utils::info!("making a public client");
        String::from("0.0.0.0:0")
    } else {
        utils::info!("making a localhost client");
        String::from("127.0.0.1:0")
    }
}

pub fn server_address(port: u16) -> String {
    let address = {
        if let Ok(ip) = std::env::var(IP_ENVVAR) {
            utils::info!("connecting to a remote server from ${IP_ENVVAR}");
            format!("{ip}:{port}")
        } else {
            utils::info!("connecting to localhost (empty ${IP_ENVVAR})");
            format!("127.0.0.1:{port}")
        }
    };

    address
}

