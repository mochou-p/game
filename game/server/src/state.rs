// mochou-p/game/game/server/src/state.rs

use std::net::SocketAddr;
use tokio::sync::RwLock;
use game_protocol::Nonce;
use super::client::Clients;


pub struct State {
    pub clients:            RwLock<Clients>,
    pub pending_challenges: RwLock<utils::Map<SocketAddr, Nonce>>
}

impl State {
    pub fn new() -> Self {
        Self {
            clients:            RwLock::new(   Clients::new()),
            pending_challenges: RwLock::new(utils::Map::new())
        }
    }
}

