use crate::game::Game;
use crate::server::Server;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;
use std::sync::{mpsc, Arc};
use crate::packet_processor::PacketProcessor;

mod game;
mod server;
mod schema_generated;
mod packet_transfer;
mod packet_processor;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        ]
    ).unwrap();

    info!("Starting MapTSServer");

    let (game_tx, game_rx) = mpsc::channel();
    let (server_tx, server_rx) = mpsc::channel();
    let game = Box::new(Game::new());
    let game_handler = game.run(server_tx);
    let packet_processor = Arc::new(PacketProcessor::new(game));
    let packet_processor_handler = packet_processor.run(game_rx);
    let server_handler = Server::new().run(packet_processor, server_rx, game_tx);
    game_handler.join().unwrap();
}
