// mochou-p/game/game/client/src/main.rs

mod game;
mod network;

use tokio::sync::{mpsc, watch};


fn main() {
    let ( g2n_w,  g2n_r) =  mpsc::channel(   16);
    let ( n2g_w,  n2g_r) =  mpsc::channel(   16);
    let (stop_w, stop_r) = watch::channel(false);

    if let Some(net) = network::spawn(n2g_w, g2n_r, stop_r) {
        game::run(g2n_w, n2g_r);
        stop_w.send_replace(true);

        if let Err(error) = net.join() {
            utils::error!("failed to join network thread: {error:?}");
        }
    }
}

