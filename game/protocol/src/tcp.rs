// mochou-p/game/game/protocol/src/tcp.rs

use serde::{Serialize, Deserialize};


pub const PORT: u16 = 10079;

pub const MAX_SERVER_LEN: u16 = 512;
pub const MAX_CLIENT_LEN: u16 = 512;

#[derive(Serialize, Deserialize)]
pub enum ServerToClient {
    Handshake { token: u128 }
}

#[derive(Serialize, Deserialize)]
pub enum ClientToServer {
    LetsShakeHands
}

