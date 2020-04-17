use std::net::{TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::schema_generated::packet::{get_root_as_packet, Packet};

pub struct Client {
    stream: Arc<Mutex<Box<TcpStream>>>,
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        stream.set_nonblocking(true);
        return Client { stream: Arc::new(Mutex::new(Box::new(stream))), buffer: Arc::new(Mutex::new(vec![])) };
    }

    fn handle_new_data(data: [u8; 50], size: u32, buffer: &mut Vec<u8>) -> Option<Packet> {
        buffer.push(data[0..size]);
        let magic_first_byte = buffer.iter().position(&0x45u8);
        match magic_first_byte {
            Some(position) => {
                match buffer.get(position + 1) {
                    Some(value) => {
                        if value == 0xFE {
                            let data_packet = &buffer[0..position];
                            return Some(get_root_as_packet(data_packet))
                        }
                    }
                    None => {}
                }
            }
            None => {}
        } None
    }

    pub fn run(&self) {
        let stream = self.stream.clone();
        let buffer = self.buffer.clone();
        thread::spawn(move || {
            let mut data = [0 as u8; 50];
            while match stream.lock().read(&mut data) {
                Ok(size) => {
                    Client::handle_new_data(data, size, buffer.lock().unwrap().as_mut())
                }
                Err(_) => {
                    error!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                    stream.shutdown(Shutdown::Both).unwrap();
                }
            } {}
        })()
    }

    pub fn send(&self) {
        thread::spawn(move || {})
    }
}