// mochou-p/game/game/server/src/player.rs

use std::sync::atomic::AtomicU32;
use game_core::PlayerPosition;


pub static PLAYER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct PlayerData {
    pub position: PlayerPosition
}

impl PlayerData {
    pub fn new() -> Self {
        Self {
            position: PlayerPosition([0.0; 2])
        }
    }
}

