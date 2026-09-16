// mochou-p/game/game/server/src/udp.rs

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::watch::Receiver;
use super::client::Clients;
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
                    let mut cs                 = state.clients.write().await;
                    let     token_from_udpaddr = cs.udpaddr2token.get(&address).cloned();

                    if let Some(_token) = token_from_udpaddr {
                        drop(cs);
                        known_token4udpaddr().await
                    } else {
                        unknown_token4udpaddr(address, state.clone(), &mut cs, incoming).await
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

async fn known_token4udpaddr() -> Option<game_protocol::udp::ServerToClient> {
    utils::todo!("have the token for this udp known");
    None
}

async fn unknown_token4udpaddr(
    address: SocketAddr,
    state:   Arc<State>,
    cs:      &mut Clients,
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
        confirm_token(address, cs, &mut pc, message).await
    }
}

async fn challenge_token(
    address:    SocketAddr,
    cs:         &mut Clients,
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

async fn confirm_token(
    address: SocketAddr,
    cs:      &mut Clients,
    pc:      &mut utils::Map<SocketAddr, game_protocol::Nonce>,
    message: game_protocol::udp::ClientToServer
) -> Option<game_protocol::udp::ServerToClient> {
    if let game_protocol::udp::ClientToServer::TokenConfirmation { token } = message {
        let client_maybe = cs.token2client.get(&token).cloned();

        if let Some(client) = client_maybe {
            {
                let mut c = client.write().await;

                if c.udp.is_some() {
                    utils::warning!("{address}: tried to confirm a token with an already known udp address");
                    return None;
                } else {
                    c.udp = Some(address);
                }
            }

            cs.udpaddr2token.insert(address, token);

            utils::ok!("{address}: token confirmed");
        } else {
            utils::warning!("{address}: unknown token provided");
        }

        None
    } else {
        utils::warning!("{address}: not TokenConfirmation, got: {message:?}");
        utils::info!("{address}: challenging");

        let Some(rng) = utils::csprng() else {
            return None;
        };

        let nonce = game_protocol::Nonce(rng);
        pc.insert(address, nonce);

        Some(game_protocol::udp::ServerToClient::TokenChallenge { nonce })
    }
}

