// mochou-p/game/game/server/src/client.rs

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;
use game_core::PlayerId;
use game_protocol::Token;
use game_protocol::tcp::ServerToClient;


pub struct Clients {
    pub token2client:  utils::Map<Token,      Arc<RwLock<Client>>>,
    pub udpaddr2token: utils::Map<SocketAddr, Token>
}

impl Clients {
    pub fn new() -> Self {
        Self {
            token2client:  utils::Map::new(),
            udpaddr2token: utils::Map::new()
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub player: Option<PlayerId>,
    pub tcp:    Sender<game_protocol::tcp::ServerToClient>,
    pub udp:    Option<SocketAddr>
}

impl Client {
    pub fn new(tcp: Sender<ServerToClient>) -> Self {
        Self { player: None, tcp, udp: None }
    }
}

