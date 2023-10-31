pub mod play_clientbound;
pub mod play_serverbound;
pub mod serializer;
pub mod config;
pub use minecraft_packet_derive::*;
use serializer::*;
use std::{convert::TryFrom, collections::BTreeMap};
pub mod handshake;
pub mod login;
pub mod status;

#[derive(Debug, Clone)]
pub struct VarInt(pub i32);
impl TryFrom<VarInt> for usize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.0)
    }
}
impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

impl From<u32> for VarInt {
    fn from(value: u32) -> Self {
        VarInt(value as i32)
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

#[derive(Debug, Clone)]
pub struct VarLong(pub i64);
impl TryFrom<VarLong> for usize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: VarLong) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.0)
    }
}
impl TryFrom<usize> for VarLong {
    type Error = std::num::TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value: i64 = TryFrom::try_from(value)?;
        Ok(VarLong(value))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[derive(Debug, PartialEq, Clone, MinecraftPacketPart)] 
pub struct GlobalPosition<'a> {
    dimension: Identifier<'a>,
    position: Position,
}

#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    South = 1,
    West,
    North,
    East,
}

pub type UUID = u128;
pub type Angle = u8;
/// Json encoded data, stored in a String.
/// See [the wiki](https://wiki.vg/Chat).
pub type Chat<'a> = &'a str; /// TODO: Check is 
/// Identifiers are a namespaced location, in the form of `minecraft:thing`.
/// If the namespace is not provided, it defaults to `minecraft` (i.e. thing is `minecraft:thing`).
/// Custom content should always be in its own namespace, not the default one.
/// The namespace should only use the characters `01​​234​5​6​78​9abcdefghijklmnopqrstuvwxyz-_`; actual names may contain more symbols.
/// The naming convention is `lower_case_with_underscores`. [More information](https://minecraft.net/en-us/article/minecraft-snapshot-17w43a).
pub type Identifier<'a> = &'a str;

/// This is used to replace an unsupported structure by taking all the remaining bytes of a packet.
/// Feel free to make PRs.
#[derive(Debug, Default)]
pub struct RawBytes<'a> {
    pub data: &'a [u8],
}

#[derive(Debug, MinecraftPacketPart)]
pub struct TestPacket {
    data: u8,
}

pub struct Array<'a, T: MinecraftPacketPart<'a> + std::fmt::Debug, U: MinecraftPacketPart<'a>> {
    _len_prefix: std::marker::PhantomData<&'a U>,
    pub items: Vec<T>,
}

impl<'a, T: MinecraftPacketPart<'a> + std::fmt::Debug, U: MinecraftPacketPart<'a>> std::fmt::Debug for Array<'a, T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.fmt(f)
    }
}

impl<'a, T: MinecraftPacketPart<'a> + std::fmt::Debug, U: MinecraftPacketPart<'a>> std::default::Default for Array<'a, T, U> {
    fn default() -> Self {
        Self { _len_prefix: std::marker::PhantomData, items: Vec::default() }
    }
}

impl<'a, T: std::fmt::Debug + MinecraftPacketPart<'a>, U: MinecraftPacketPart<'a>> From<Vec<T>> for Array<'a, T, U> {
    fn from(value: Vec<T>) -> Self {
        Array {
            _len_prefix: std::marker::PhantomData,
            items: value,
        }
    }
}

pub struct Map<
    'a,
    K: MinecraftPacketPart<'a> + std::fmt::Debug,
    V: MinecraftPacketPart<'a> + std::fmt::Debug,
    U: MinecraftPacketPart<'a>,
> {
    _len_prefix: std::marker::PhantomData<&'a U>,
    pub items: BTreeMap<K, V>,
}

impl<'a, K: std::fmt::Debug + MinecraftPacketPart<'a>, V: std::fmt::Debug + MinecraftPacketPart<'a>, U: MinecraftPacketPart<'a>> std::fmt::Debug for Map<'a, K, V, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.items.fmt(f)
    }
}

impl<'a, K: std::fmt::Debug + MinecraftPacketPart<'a>, V: std::fmt::Debug + MinecraftPacketPart<'a>, U: MinecraftPacketPart<'a>> std::default::Default for Map<'a, K, V, U> {
    fn default() -> Self {
        Self { _len_prefix: std::marker::PhantomData, items: BTreeMap::default() }
    }
}

impl<'a, K: std::fmt::Debug + MinecraftPacketPart<'a>, V: std::fmt::Debug + MinecraftPacketPart<'a>, U: MinecraftPacketPart<'a>> From<BTreeMap<K, V>> for Map<'a, K, V, U> {
    fn from(value: BTreeMap<K, V>) -> Self {
        Map {
            _len_prefix: std::marker::PhantomData,
            items: value,
        }
    }
}

/// The possible packets are different for each state.
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ConnectionState {
    /// The possible packets are listed in [handshake].
    HandShake,
    /// The possible packets are listed in [status].
    Status,
    /// The possible packets are listed in [login].
    Login,
    /// The possible packets are listed in [play_clientbound] and [play_serverbound].
    Play,
}
