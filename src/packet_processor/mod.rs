use std::thread;
use crate::game::Game;
use crate::packet_transfer::PacketTransfer;
use std::sync::mpsc::Receiver;
use std::sync::{Mutex, Arc};
use crate::schema_generated::packet::{get_root_as_packet, PacketType};
use std::thread::JoinHandle;

pub struct PacketProcessor {
    game: Arc<Mutex<Box<Game>>>
}

impl PacketProcessor {
    pub fn new(game: Box<Game>) -> Self {
        return PacketProcessor {
            game: Arc::new(Mutex::new(game))
        };
    }

    pub fn run(&self, game_rx: Receiver<PacketTransfer>) -> JoinHandle<()> {
        return thread::spawn(move || {
            while match game_rx.recv() {
                Ok(packet_transfert) => {
                    true
                }
                Err(_) => { false }
            } {}
        });
    }

    pub fn check_login_packet(&self, buffer: &[u8]) -> Option<u32> {
        let packet = get_root_as_packet(buffer);
        if packet.data_type() == PacketType::LoginPacket {
            match self.game.lock().unwrap().players.lock().unwrap().get(0) {
                Some(player) => { Some(player.id) }
                None => { None }
            }
        } else { None }
    }
}
