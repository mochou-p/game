// mochou-p/game/game/protocol/src/tcp.rs

use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpStream;


pub const PORT: u16 = 10079;

pub const MAX_SERVER_LEN: u16 = 512;
pub const MAX_CLIENT_LEN: u16 = 512;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerToClient {
    Handshake { token: u32 }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientToServer {
    LetsShakeHands
}

pub async fn send_s2c(
    client:  &mut TcpStream,
    address: SocketAddr,
    message: ServerToClient,
    buffer:  &mut [u8; MAX_SERVER_LEN as usize]
) -> bool {
    let bytes = match postcard::to_slice(&message, buffer) {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to serialize message {message:?}: {err}");
            return false;
        }
    };

    let length = bytes.len();
    if length == 0 || length > MAX_SERVER_LEN as usize {
        utils::error!("invalid length of serialized message: {length}");
        return false;
    }

    if let Err(err) = client.write_u16(length as u16).await {
        utils::warning!("{address}: failed to write u16: {err}");
        return false;
    }

    if let Err(err) = client.write_all(bytes).await {
        utils::warning!("{address}: failed to write bytes: {err}");
        return false;
    }

    true
}

pub async fn recv_c2s(
    client:  &mut TcpStream,
    address: SocketAddr,
    buffer:  &mut [u8; MAX_CLIENT_LEN as usize]
) -> Option<ClientToServer> {
    let length = match client.read_u16().await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::warning!("{address}: failed to read length: {err}");
            return None;
        }
    };

    if length == 0 || length > MAX_CLIENT_LEN {
        utils::warning!("{address}: invalid length frame: {length}");
        return None;
    }
    let length = length as usize;

    if let Err(err) = client.read_exact(&mut buffer[..length]).await {
        utils::warning!("{address}: failed to read {length} bytes: {err}");
        return None;
    }

    match postcard::from_bytes(&buffer[..length]) {
        Ok (ok ) => Some(ok),
        Err(err) => {
            utils::warning!("{address}: failed to deserialize bytes {:?}: {err}", &buffer[..length]);
            None
        }
    }
}

pub async fn send_c2s(
    server:  &mut TcpStream,
    message: ClientToServer,
    buffer:  &mut [u8; MAX_CLIENT_LEN as usize]
) -> bool {
    let bytes = match postcard::to_slice(&message, buffer) {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to serialize message {message:?}: {err}");
            return false;
        }
    };

    let length = bytes.len();
    if length == 0 || length > MAX_CLIENT_LEN as usize {
        utils::error!("invalid length of serialized message: {length}");
        return false;
    }

    if let Err(err) = server.write_u16(length as u16).await {
        utils::warning!("failed to write u16: {err}");
        return false;
    }

    if let Err(err) = server.write_all(bytes).await {
        utils::warning!("failed to write bytes: {err}");
        return false;
    }

    true
}

pub async fn recv_s2c(
    server: &mut TcpStream,
    buffer: &mut [u8; MAX_CLIENT_LEN as usize]
) -> Option<ServerToClient> {
    let length = match server.read_u16().await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::warning!("failed to read length: {err}");
            return None;
        }
    };

    if length == 0 || length > MAX_SERVER_LEN {
        utils::warning!("invalid length frame: {length}");
        return None;
    }
    let length = length as usize;

    if let Err(err) = server.read_exact(&mut buffer[..length]).await {
        utils::warning!("failed to read {length} bytes: {err}");
        return None;
    }

    match postcard::from_bytes(&buffer[..length]) {
        Ok (ok ) => Some(ok),
        Err(err) => {
            utils::warning!("failed to deserialize bytes {:?}: {err}", &buffer[..length]);
            None
        }
    }
}

