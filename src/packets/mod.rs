pub mod serializer;
pub mod play_clientbound;
pub use minecraft_packet_derive::*;
use serializer::*;

#[derive(Debug)]
pub struct VarInt(pub i32);
#[derive(Debug)]
pub struct VarLong(pub i64);

pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

pub type UUID = u128;
pub type Angle = u8;

#[derive(Debug, MinecraftPacket)]
pub struct TestPacket {
    data: u8,
}
