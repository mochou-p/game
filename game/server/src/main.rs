// mochou-p/game/game/server/src/main.rs

mod tcp;
mod udp;


#[tokio::main]
async fn main() {
    database_core::setup();

    let (stop_write, stop_read) = tokio::sync::watch::channel(false);

    let tcp = tokio::spawn(tcp::bind(stop_read.clone()));
    let udp = tokio::spawn(udp::bind(stop_read        ));

    tokio::signal::ctrl_c().await.unwrap();
    stop_write.send(true).unwrap();
    println!();

    let (tcp_result, udp_result) = tokio::join!(tcp, udp);

    tcp_result.unwrap();
    udp_result.unwrap();
}

