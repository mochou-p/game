// mochou-p/game/game/client/src/main.rs

mod game;
mod network;


fn main() {
    let (stop_write, stop_read) = tokio::sync::watch::channel(false);

    let network = network::spawn(stop_read);

    game::run();
    stop_write.send(true).unwrap();

    network.join().unwrap();
}

