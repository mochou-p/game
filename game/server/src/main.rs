// mochou-p/game/game/server/src/main.rs

mod client;
mod state;
mod tcp;
mod udp;

use std::sync::Arc;


#[tokio::main]
async fn main() {
    if let Err(err) = database_core::setup() {
        utils::error!("failed to setup database: {err}");
        return;
    }

    let state                   = Arc::new(state::State::new());
    let (stop_write, stop_read) = tokio::sync::watch::channel(false);

    let tcp = tokio::spawn(tcp::bind(state.clone(), stop_read.clone()));
    let udp = tokio::spawn(udp::bind(state,         stop_read        ));

    if let Err(err) = tokio::signal::ctrl_c().await {
        utils::error!("SIGINT signal failed: {err}");
    }

    stop_write.send_replace(true);
    println!();

    let (tcp_result, udp_result) = tokio::join!(tcp, udp);

    if let Err(err) = tcp_result {
        utils::error!("tcp task crashed: {err}");
    }

    if let Err(err) = udp_result {
        utils::error!("udp task crashed: {err}");
    }
}

