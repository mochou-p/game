// mochou-p/game/game/protocol/src/udp.rs

use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use tokio::net::UdpSocket;


pub const PORT: u16 = 10089;

pub const MAX_SERVER_LEN: u16 = 512;
pub const MAX_CLIENT_LEN: u16 = 512;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerToClient {
    Temp
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientToServer {
    Temp
}

pub async fn send_s2c(
    server:  &mut UdpSocket,
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

    if let Err(err) = server.send_to(bytes, address).await {
        utils::error!("failed to write bytes to {address}: {err}");
        return false;
    }

    true
}

pub async fn recv_c2s(
    server:  &mut UdpSocket,
    buffer:  &mut [u8; MAX_CLIENT_LEN as usize]
) -> Option<(SocketAddr, ClientToServer)> {
    let (length, address) = match server.recv_from(buffer).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to receive a message: {err}");
            return None;
        }
    };

    if length == 0 || length > MAX_CLIENT_LEN as usize {
        utils::warning!("{address}: invalid length frame: {length}");
        return None;
    }

    match postcard::from_bytes(&buffer[..length]) {
        Ok (ok ) => Some((address, ok)),
        Err(err) => {
            utils::warning!("{address}: failed to deserialize bytes {:?}: {err}", &buffer[..length]);
            None
        }
    }
}

pub async fn send_c2s(
    server:  &mut UdpSocket,
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

    if let Err(err) = server.send(bytes).await {
        utils::error!("failed to write bytes: {err}");
        return false;
    }

    true
}

pub async fn recv_s2c(
    server: &mut UdpSocket,
    buffer: &mut [u8; MAX_SERVER_LEN as usize]
) -> Option<ServerToClient> {
    let length = match server.recv(buffer).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to receive a message: {err}");
            return None;
        }
    };

    if length == 0 || length > MAX_SERVER_LEN as usize {
        utils::warning!("invalid length frame: {length}");
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

