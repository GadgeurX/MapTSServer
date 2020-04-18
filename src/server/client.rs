use std::net::{TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::prelude::*;
use crate::schema_generated::packet::{get_root_as_packet, Packet, OkPacketArgs, PacketArgs, PacketType, finish_packet_buffer, OkPacketOffset, OkPacketBuilder};
use crate::packet_transfer::PacketTransfer;
use crate::packet_processor::PacketProcessor;
use std::sync::mpsc::Sender;
use flatbuffers::FlatBufferBuilder;
use crate::schema_generated::packet::PacketType::OkPacket;
use crate::schema_generated::packet;

pub struct Client {
    pub stream: Arc<Mutex<Box<TcpStream>>>,
    pub buffer: Arc<Mutex<Vec<u8>>>,
    pub player_id: Arc<Mutex<Option<u32>>>,
    pub packet_processor: Arc<Mutex<Arc<PacketProcessor>>>
}

impl Client {
    pub fn new(stream: TcpStream, packet_processor: Arc<PacketProcessor>) -> Self {
        stream.set_nonblocking(true);
        return Client {
            stream: Arc::new(Mutex::new(Box::new(stream))),
            buffer: Arc::new(Mutex::new(vec![])),
            player_id: Arc::new(Mutex::new(None)),
            packet_processor: Arc::new(Mutex::new(packet_processor))
        };
    }

    fn handle_new_data(data: [u8; 50], size: usize, buffer: &mut Vec<u8>) -> Option<PacketTransfer> {
        buffer.extend(data[0..size].iter());
        let magic_first_byte = buffer.iter().position(|&x| x == 0x45u8);
        match magic_first_byte {
            Some(position) => {
                match buffer.get(position + 1) {
                    Some(value) => {
                        if *value == 0xFEu8 {
                            let data_packet = vec![0; position];
                            &buffer[0..position].copy_from_slice(&data_packet);
                            buffer.drain(0..=position + 1);
                            return Some(PacketTransfer { player_id: 0, buffer: data_packet });
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
        None
    }

    pub fn run(&self, server_tx: Sender<PacketTransfer>) {
        let stream = self.stream.clone();
        let buffer = self.buffer.clone();
        let packet_processor = self.packet_processor.clone();
        let player_id = self.player_id.clone();
        thread::spawn(move || {
            let mut data = [0 as u8; 50];
            while match stream.lock().unwrap().as_mut().read(&mut data) {
                Ok(size) => {
                    let packet_transfer = Client::handle_new_data(data, size, buffer.lock().unwrap().as_mut());
                    match packet_transfer {
                        Some(packet_transfer) => {
                            match packet_processor.lock().unwrap().check_login_packet(&packet_transfer.buffer) {
                                Some(player) => {
                                    player_id.lock().unwrap().replace(player);
                                    let mut bldr = FlatBufferBuilder::new();
                                    let mut bytes: Vec<u8> = Vec::new();
                                    let ok_packet = packet::OkPacket::create(&mut bldr, &OkPacketArgs{});
                                    let packet = Packet::create(&mut bldr, &PacketArgs {
                                        data_type: PacketType::OkPacket,
                                        data: Some(ok_packet.as_union_value())
                                    });
                                    finish_packet_buffer(&mut bldr, packet);
                                    let finished_data = bldr.finished_data();
                                    bytes.extend_from_slice(finished_data);
                                    stream.lock().unwrap().as_mut().write(&bytes);
                                }
                                None => {
                                    server_tx.send(packet_transfer);
                                }
                            }
                        }
                        None => {}
                    }
                    true
                }
                Err(_) => {
                    error!("An error occurred, terminating connection with {}", stream.lock().unwrap().as_ref().peer_addr().unwrap());
                    stream.lock().unwrap().as_ref().shutdown(Shutdown::Both).unwrap();
                    false
                }
            } {}
        });
    }

    pub fn send(&self, buffer: &[u8]) {
        let stream = self.stream.clone();
        stream.lock().unwrap().as_mut().write(buffer);
    }
}