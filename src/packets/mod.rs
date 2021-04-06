pub mod serializer;
use minecraft_packet_derive::*;
use serializer::*;

#[derive(PartialEq, Debug)]
pub struct VarInt(pub i32);
pub struct VarLong(pub i64);

#[derive(Debug, MinecraftPacket)]
pub struct TestPacket {
    data: u8,
}

