#![allow(clippy::upper_case_acronyms)]

pub mod animations;
pub mod digging_states;
pub mod nbt;
pub mod packets;
pub mod paintings;

pub(crate) use minecraft_packet_derive::minecraft_enum;
pub(crate) use crate::packets::serializer::MinecraftPacketPart;
pub(crate) use crate::packets::VarInt;