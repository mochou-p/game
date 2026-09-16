// mochou-p/game/game/protocol/src/lib.rs

pub mod tcp;
pub mod udp;

use serde::{Serialize, Deserialize};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token(pub [u8; 32]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Nonce(pub [u8; 16]);

