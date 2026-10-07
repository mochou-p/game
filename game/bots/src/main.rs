// mochou-p/game/game/bots/src/main.rs

#[tokio::main]
async fn main() {
    let count     = ask("how many clients to spawn",                ""        );
    let duration1 = ask("how long to wait between client spawns",   " seconds");
    let duration2 = ask("how long to wait between client movement", " seconds");

    let client_count        = count;
    let wait_between_spawns = std::time::Duration::from_secs_f32(duration1);
    let movement_interval   = std::time::Duration::from_secs_f32(duration2);

    for i in 0..client_count {
        if i != 0 {
            tokio::time::sleep(wait_between_spawns).await;
        }

        tokio::spawn(bot(movement_interval));
    }

    let _ = tokio::signal::ctrl_c().await;
    println!();
}

fn ask<T: std::str::FromStr>(message: &str, units: &str) -> T
where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    use std::io::Write as _;

    print!("{message}: ({}{units}) ", std::any::type_name::<T>());
    std::io::stdout().flush().unwrap();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();

    buffer[..buffer.len() - 1].parse().unwrap()
}

async fn bot(interval: std::time::Duration) {
    let mut tcp_read_buffer  = [0; game_protocol::tcp::MAX_SERVER_LEN as usize];
    let mut tcp_write_buffer = [0; game_protocol::tcp::MAX_CLIENT_LEN as usize];
    let mut udp_read_buffer  = [0; game_protocol::udp::MAX_SERVER_LEN as usize];
    let mut udp_write_buffer = [0; game_protocol::udp::MAX_CLIENT_LEN as usize];

    let mut tcp = tokio::net::TcpStream::connect(
        game_protocol::server_address(game_protocol::tcp::PORT)
    ).await.unwrap();

    let mut udp = tokio::net::UdpSocket::bind(
        game_protocol::client_address()
    ).await.unwrap();

    udp.connect(
        game_protocol::server_address(game_protocol::udp::PORT)
    ).await.unwrap();

    assert!(
        game_protocol::tcp::send_c2s(
            &mut tcp,
            game_protocol::tcp::ClientToServer::LetsShakeHands,
            &mut tcp_write_buffer
        ).await
    );

    let message = game_protocol::tcp::recv_s2c(
        &mut tcp,
        &mut tcp_read_buffer
    ).await.unwrap();

    let game_protocol::tcp::ServerToClient::Handshake { token } = message else {
        panic!();
    };

    assert!(
        game_protocol::udp::send_c2s(
            &mut udp,
            game_protocol::udp::ClientToServer::TokenConfirmation { token },
            &mut udp_write_buffer
        ).await
    );

    let message = game_protocol::udp::recv_s2c(
        &mut udp,
        &mut udp_read_buffer
    ).await.unwrap();

    assert!(matches!(message, game_protocol::udp::ServerToClient::TokenConfirmed));

    let mut rng = fastrand::Rng::new();
    let     w   = 800.0;
    let     h   = 600.0;

    loop {
        let x = (rng.f32() - 0.5) * w;
        let y = (rng.f32() - 0.5) * h;

        let position = game_core::PlayerPosition([x, y]);

        assert!(
            game_protocol::udp::send_c2s(
                &mut udp,
                game_protocol::udp::ClientToServer::PositionChanged { position },
                &mut udp_write_buffer
            ).await
        );

        tokio::time::sleep(interval).await;
    }
}

