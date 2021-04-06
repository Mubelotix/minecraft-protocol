pub mod serializer;
use minecraft_packet_derive::*;
use serializer::*;

pub struct VarInt(pub i32);
pub struct VarLong(pub i64);

pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

type UUID = u128;
type Angle = u8;

#[derive(Debug, MinecraftPacket)]
pub struct TestPacket {
    data: u8,
}

