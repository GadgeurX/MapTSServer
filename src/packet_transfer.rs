use crate::schema_generated::packet::Packet;

pub struct PacketTransfer {
    pub player_id: u32,
    pub buffer: Vec<u8>
}