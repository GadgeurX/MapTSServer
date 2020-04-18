use std::thread;
use std::thread::JoinHandle;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::schema_generated::packet::get_root_as_packet;
use std::sync::{Arc, Mutex};
use crate::server::client::Client;
use std::sync::mpsc::{Sender, Receiver};
use crate::packet_transfer::PacketTransfer;
use crate::packet_processor::PacketProcessor;

mod client;

pub struct Server {
    pub clients: Arc<Mutex<Vec<Box<Client>>>>,
}

impl Server {
    pub fn new() -> Self {
        info!("Creating Server");
        return Server { clients: Arc::new(Mutex::new(vec![])) };
    }

    fn handle_new_client(packet_processor: Arc<PacketProcessor>, stream: TcpStream, clients: &mut Vec<Box<Client>>, server_tx: Sender<PacketTransfer>) {
        let client = Box::new(Client::new(stream, packet_processor));
        client.run(server_tx);
        clients.push(client)
    }

    pub fn run(&self, packet_processor: Arc<PacketProcessor>, game_rx: Receiver<PacketTransfer>, server_tx: Sender<PacketTransfer>) -> JoinHandle<()> {
        let clients = self.clients.clone();
        let clients_bridge = self.clients.clone();
        thread::spawn(move || {
            while match game_rx.recv() {
                Ok(packet_transfer) => {
                    match clients_bridge.lock().unwrap().iter().find(|client| {
                        match client.player_id.lock().unwrap().as_mut() {
                            Some(player_id) => {
                                if *player_id == packet_transfer.player_id {
                                    true
                                } else { false }
                            }
                            None => { false }
                        }
                    }) {
                        Some(client) => {
                            client.send(&packet_transfer.buffer);
                        }
                        None => {}
                    }
                    true
                }
                Err(_) => {
                    false
                }
            } {}
        });
        return thread::spawn(
            move || {
                info!("Starting Server");
                loop {
                    let listener = TcpListener::bind("0.0.0.0:8642").unwrap();
                    info!("Server listening on port 8642");
                    for stream in listener.incoming() {
                        match stream {
                            Ok(stream) => {
                                info!("New connection: {}", stream.peer_addr().unwrap());
                                Server::handle_new_client(packet_processor.clone(), stream, clients.lock().unwrap().as_mut(), server_tx.clone())
                            }
                            Err(e) => {
                                error!("Error: {}", e);
                            }
                        }
                    }
                    drop(listener);
                }
            }
        );
    }
}