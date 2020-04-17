use crate::game::Game;
use crate::server::Server;

#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;
mod game;
mod server;
mod schema_generated;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
        ]
    ).unwrap();

    info!("Starting MapTSServer");
    let game_handler = Game::new().run();
    let server_handler = Server::new().run();
    game_handler.join().unwrap();
}
