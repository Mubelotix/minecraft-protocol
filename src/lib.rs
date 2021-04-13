#![allow(clippy::upper_case_acronyms)]

pub mod advancements;
pub mod animations;
pub mod auto_completion;
pub mod blocks;
pub mod boss_bar;
pub mod chat;
pub mod chunk;
pub mod combat;
pub mod command_block;
pub mod difficulty;
pub mod effect;
pub mod entity;
pub mod game_state;
pub mod gamemode;
pub mod network;
pub mod nbt;
pub mod packets;
pub mod paintings;
pub mod players;
pub mod recipes;
pub mod resource_pack;
pub mod slots;
pub mod sound;
pub mod teams;
pub mod trades;
pub mod ids;

pub(crate) use crate::packets::serializer::MinecraftPacketPart;
pub(crate) use crate::packets::*;
