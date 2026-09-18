// mochou-p/game/game/server/src/state.rs

use std::net::SocketAddr;
use tokio::sync::RwLock;
use game_core::PlayerId;
use game_protocol::Nonce;
use super::client::Clients;
use super::player::PlayerData;


pub struct State {
    pub clients:            RwLock<Clients>,
    pub pending_challenges: RwLock<utils::Map<SocketAddr, Nonce>>,
    pub players:            RwLock<utils::Map<PlayerId, PlayerData>>
}

impl State {
    pub fn new() -> Self {
        Self {
            clients:            RwLock::new(   Clients::new()),
            pending_challenges: RwLock::new(utils::Map::new()),
            players:            RwLock::new(utils::Map::new())
        }
    }
}

