use std::thread;
use std::thread::JoinHandle;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use crate::schema_generated::packet::get_root_as_packet;
use std::sync::{Arc, Mutex};
use crate::server::client::Client;

mod client;

pub struct Server {
    clients: Arc<Mutex<Vec<Box<Client>>>>
}

impl Server {
    pub fn new() -> Self {
        info!("Creating Server");
        return Server { clients: Arc::new(Mutex::new(vec![])) };
    }

    fn handle_new_client(mut stream: TcpStream, clients: &mut Vec<Box<Client>>) {
        let client = Box::new(Client::new(stream));
        client.run();
        clients.push(client)
    }

    pub fn run(&self) -> JoinHandle<()> {
        let clients = self.clients.clone();
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
                                Server::handle_new_client(stream, clients.lock().unwrap().as_mut())
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