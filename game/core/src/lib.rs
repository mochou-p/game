// mochou-p/game/game/core/src/lib.rs

use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerPosition(pub [f32; 2]);

