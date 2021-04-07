#![allow(clippy::upper_case_acronyms)]

pub mod animations;
pub mod blocks;
pub mod boss_bar;
pub mod difficulty;
pub mod nbt;
pub mod packets;
pub mod paintings;

pub(crate) use crate::packets::serializer::MinecraftPacketPart;
pub(crate) use crate::packets::*;
pub(crate) use minecraft_packet_derive::minecraft_enum;
pub(crate) use minecraft_packet_derive::MinecraftStructuredEnum;
