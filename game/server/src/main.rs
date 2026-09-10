// mochou-p/game/game/server/src/main.rs

mod tcp;
mod udp;


#[tokio::main]
async fn main() {
    if let Err(err) = database_core::setup() {
        utils::error!("failed to setup database: {err}");
        return;
    }

    let (stop_write, stop_read) = tokio::sync::watch::channel(false);

    let tcp = tokio::spawn(tcp::bind(stop_read.clone()));
    let udp = tokio::spawn(udp::bind(stop_read        ));

    if let Err(error) = tokio::signal::ctrl_c().await {
        utils::error!("SIGINT signal failed: {error}");
    }

    stop_write.send_replace(true);
    println!();

    let (tcp_result, udp_result) = tokio::join!(tcp, udp);

    if let Err(error) = tcp_result {
        utils::error!("tcp task crashed: {error}");
    }

    if let Err(error) = udp_result {
        utils::error!("udp task crashed: {error}");
    }
}

