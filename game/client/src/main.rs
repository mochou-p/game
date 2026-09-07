// mochou-p/game/game/client/src/main.rs

mod game;
mod network;

use tokio::sync::{mpsc, watch};


fn main() {
    let (game2network_write, game2network_read) =  mpsc::channel(16);
    let (network2game_write, network2game_read) =  mpsc::channel(16);
    let (        stop_write,         stop_read) = watch::channel(false);

    let network = network::spawn(network2game_write, game2network_read, stop_read);

    game::run(game2network_write, network2game_read);
    stop_write.send(true).unwrap();

    network.join().unwrap();
}

