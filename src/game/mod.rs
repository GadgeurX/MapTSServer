use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::JoinHandle;
use crate::game::entity::unit::worker::Worker;
use std::sync::mpsc::{Receiver, Sender};
use crate::packet_transfer::PacketTransfer;

mod player;
mod entity;

pub struct Game {
    pub players: Arc<Mutex<Vec<player::Player>>>,
    pub entities: Arc<Mutex<Vec<Box<dyn entity::Entity + Send>>>>,
}

impl Game {
    pub fn new() -> Self {
        info!("Creating Game");
        return Game {
            players: Arc::new(Mutex::new(vec![])),
            entities: Arc::new(Mutex::new(vec![Box::new(Worker::new(0))])),
        };
    }

    pub fn run(&self, server_tx: Sender<PacketTransfer>) -> JoinHandle<()>{
        let entities = self.entities.clone();
        return thread::spawn(move || {
            info!("Starting Game");
            loop {
                for (entity) in entities.lock().unwrap().iter_mut() {
                    entity.update();
                }
                thread::sleep(Duration::from_millis(16));
            }
        }
        );
    }
}