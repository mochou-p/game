// mochou-p/game/game/protocol/src/udp.rs

use serde::{Serialize, Deserialize};


pub const PORT: u16 = 10089;

pub const MAX_SERVER_LEN: u16 = 512;
pub const MAX_CLIENT_LEN: u16 = 512;

#[derive(Serialize, Deserialize)]
pub enum ServerToClient {
    Temp
}

#[derive(Serialize, Deserialize)]
pub enum ClientToServer {
    Temp
}

