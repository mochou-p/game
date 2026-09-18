// mochou-p/game/game/server/src/udp.rs

use std::sync::atomic::Ordering;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio::sync::watch::Receiver;
use game_core::PlayerId;
use super::client::Clients;
use super::player::{PlayerData, PLAYER_ID_COUNTER};
use super::state::State;


pub async fn bind(
        state: Arc<State>,
    mut stop:  Receiver<bool>
) {
    const ADDRESS: [u8; 4] = [127, 0, 0, 1];

    let     address = format!("{}.{}.{}.{}:{}", ADDRESS[0], ADDRESS[1], ADDRESS[2], ADDRESS[3], game_protocol::udp::PORT);
    let mut server  = match UdpSocket::bind(address).await {
        Ok (ok ) => ok,
        Err(err) => {
            utils::error!("failed to bind socket: {err}");
            return;
        }
    };

    let mut  read_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];
    let mut write_buffer = [0; game_protocol::udp::MAX_SERVER_LEN as usize];

    utils::ok!("online");

    loop {
        // TODO: cancel safe?
        tokio::select! {
            result = game_protocol::udp::recv_c2s(&mut server, &mut read_buffer) => {
                let Some((address, incoming)) = result else {
                    continue;
                };

                utils::debug!("{address}: INCOMING: {incoming:?}");

                if let Some(outgoing) = {
                    let cs                 = state.clients.write().await;
                    let token_from_udpaddr = cs.udpaddr2token.get(&address).cloned();

                    if let Some(token) = token_from_udpaddr {
                        // TODO: well i dont need a write cs here
                        known_token4udpaddr(
                            address,
                            &mut server,
                            &mut write_buffer,
                            state.clone(),
                            cs,
                            token,
                            incoming
                        ).await;

                        None
                    } else {
                        unknown_token4udpaddr(
                            address,
                            state.clone(),
                            cs,
                            incoming
                        ).await
                    }
                } {
                    utils::debug!("{address}: OUTGOING: {outgoing:?}");

                    if !game_protocol::udp::send_s2c(
                        &mut server,
                        address,
                        outgoing,
                        &mut write_buffer
                    ).await {
                        continue;
                    }
                }
            },

            _ = stop.changed() => {
                if *stop.borrow() {
                    utils::debug!("saw ^C");
                    break;
                }
            }
        }
    }

    utils::info!("offline");
}

async fn known_token4udpaddr(
    address: SocketAddr,
    server:  &mut UdpSocket,
    buffer:  &mut [u8; game_protocol::udp::MAX_SERVER_LEN as usize],
    state:   Arc<State>,
    cs:      RwLockWriteGuard<'_, Clients>,
    token:   game_protocol::Token,
    message: game_protocol::udp::ClientToServer
) {
    match message {
        game_protocol::udp::ClientToServer::TokenConfirmation { .. }
            | game_protocol::udp::ClientToServer::TokenChallenge { .. }
        => {
            utils::warning!("{address}: got an unexpected token confirmation/challenge");
        },
        game_protocol::udp::ClientToServer::PositionChanged { position } => {
            let id = {
                if let Some(client) = cs.token2client.get(&token) {
                    let c = client.read().await;

                    if let Some(player_id) = c.player {
                        player_id
                    } else {
                        utils::warning!("{address}: client under the stored token is not a player");
                        return;
                    }
                } else {
                    utils::warning!("{address}: there is no client under the stored token");
                    return;
                }
            };

            {
                let mut ps = state.players.write().await;

                if let Some(player) = ps.get_mut(&id) {
                    // TODO: ac
                    player.position = position;
                } else {
                    utils::warning!("{address}: stored player id does not exist");
                    return;
                }
            }

            for (key, client) in &cs.token2client.0 {
                if *key != token {
                    let c = client.read().await;

                    if let Some(udp) = c.udp {
                        // TODO: really should separate serialization and transmittion
                        game_protocol::udp::send_s2c(
                            server,
                            udp,
                            game_protocol::udp::ServerToClient::PlayerPosition { id, position },
                            buffer
                        ).await;
                    }
                }
            }
        }
    }
}

async fn unknown_token4udpaddr(
    address: SocketAddr,
    state:   Arc<State>,
    cs:      RwLockWriteGuard<'_, Clients>,
    message: game_protocol::udp::ClientToServer
) -> Option<game_protocol::udp::ServerToClient> {
    let mut pc              = state.pending_challenges.write().await;
    let     challenge_maybe = pc.entry(address);

    if let utils::MapEntry::Occupied(entry) = challenge_maybe {
        if challenge_token(address, cs, *entry.get(), message).await {
            entry.remove();
        }

        None
    } else {
        confirm_token(address, cs, &mut pc, &state.players, message).await
    }
}

async fn confirm_token(
        address: SocketAddr,
    mut cs:      RwLockWriteGuard<'_, Clients>,
        pc:      &mut utils::Map<SocketAddr, game_protocol::Nonce>,
        players: &RwLock<utils::Map<PlayerId, PlayerData>>,
        message: game_protocol::udp::ClientToServer
) -> Option<game_protocol::udp::ServerToClient> {
    if let game_protocol::udp::ClientToServer::TokenConfirmation { token } = message {
        let client_maybe = cs.token2client.get(&token).cloned();

        if let Some(client) = client_maybe {
            let (channel, player_id) = {
                let mut c = client.write().await;

                if c.udp.is_some() {
                    utils::warning!("{address}: tried to confirm a token with an already known udp address");
                    return None;
                } else {
                    let player_id = PlayerId(
                        PLAYER_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
                    );

                    c.udp    = Some(address);
                    c.player = Some(player_id);

                    (c.tcp.clone(), player_id)
                }
            };

            cs.udpaddr2token.insert(address, token);

            let player_positions = {
                let     ps    = players.read().await;
                let mut pairs = Vec::with_capacity(ps.0.len());

                for (id, data) in &ps.0 {
                    pairs.push((*id, data.position));
                }

                pairs
            };

            for (id, position) in player_positions {
                let _ = channel.send(
                    game_protocol::tcp::ServerToClient::PlayerExists { id, position }
                ).await;
            }

            // TODO: broadcast, and im probably holding cs lock for too long
            //       and i also hate these reads
            let others = {
                let mut channels = Vec::with_capacity(cs.token2client.0.len());

                for (key, client) in &cs.token2client.0 {
                    if *key != token {
                        let c = client.read().await;
                        channels.push(c.tcp.clone());
                    }
                }

                channels
            };

            for other_channel in others {
                let _ = other_channel.send(
                    game_protocol::tcp::ServerToClient::PlayerJoined { id: player_id }
                ).await;
            }

            {
                let mut ps = players.write().await;
                ps.insert(player_id, PlayerData::new());
            }

            utils::ok!("{address}: token confirmed");
            Some(game_protocol::udp::ServerToClient::TokenConfirmed)
        } else {
            utils::warning!("{address}: unknown token provided");
            None
        }
    } else {
        utils::warning!("{address}: not TokenConfirmation, got: {message:?}");
        utils::info!("{address}: challenging");

        let rng   = utils::csprng()?;
        let nonce = game_protocol::Nonce(rng);
        pc.insert(address, nonce);

        Some(game_protocol::udp::ServerToClient::TokenChallenge { nonce })
    }
}

async fn challenge_token(
        address:    SocketAddr,
    mut cs:         RwLockWriteGuard<'_, Clients>,
        real_nonce: game_protocol::Nonce,
        message:    game_protocol::udp::ClientToServer
) -> bool {
    if let game_protocol::udp::ClientToServer::TokenChallenge { token, nonce } = message {
        if nonce != real_nonce {
            utils::warning!("{address}: wrong nonce");
            return false;
        }

        let Some(client) = cs.token2client.get(&token) else {
            utils::warning!("{address}: wrong token");
            return false;
        };

        // NOTE: could also log tcp addr or client name or player id
        let old_address = {
            let mut c = client.write().await;

            if let Some(stored_address) = c.udp.as_mut() {
                let old         = *stored_address;
                *stored_address = address;
                old
            } else {
                utils::warning!("{address}: tried to challenge a token with an unknown udp address");
                return false;
            }
        };

        cs.udpaddr2token.insert(address, token);
        cs.udpaddr2token.remove(&old_address);

        utils::ok!("{address}: challenge passed, replaced old udp address {old_address}");

        true
    } else {
        utils::warning!("{address}: expected TokenChallenge, got: {message:?}");
        false
    }
}

