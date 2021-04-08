#![allow(clippy::upper_case_acronyms)]

pub mod animations;
pub mod auto_completion;
pub mod blocks;
pub mod boss_bar;
pub mod chat;
pub mod chunk;
pub mod combat;
pub mod difficulty;
pub mod game_state;
pub mod gamemode;
pub mod nbt;
pub mod packets;
pub mod paintings;
pub mod slots;
pub mod trades;

pub(crate) use crate::packets::serializer::MinecraftPacketPart;
pub(crate) use crate::packets::*;
pub(crate) use minecraft_packet_derive::*;
