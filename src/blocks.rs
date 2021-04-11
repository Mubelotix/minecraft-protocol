use crate::*;

#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum PartialDiggingState {
    Started,
    Cancelled,
    Finished,
}

/// See [the wiki](https://wiki.vg/Protocol#Player_Digging)
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum DiggingState {
    Started,
    Cancelled,
    Finished,
    DropItemStack,
    DropItem,
    ShootArrowOrFinishEating,
    SwapItemInHand,
}

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

/// The type of update to perform used in [crate::packets::play_clientbound::BlockEntityData] packets.
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum BlockEntityDataAction {
    /// Set data of a mob spawner (everything except for SpawnPotentials: current delay, min/max delay, mob to be spawned, spawn count, spawn range, etc.)
    MobSpawner = 1,
    /// Set command block text (command and last execution status)
    CommandBlock,
    /// Set the level, primary, and secondary powers of a beacon
    Beacon,
    /// Set rotation and skin of mob head
    MobHead,
    /// Declare a conduit
    Conduit,
    /// Set base color and patterns on a banner
    Banner,
    /// Set the data for a Structure tile entity
    Structure,
    /// Set the destination for a end gateway
    EndGateway,
    /// Set the text on a sign
    Sign,

    /// Declare a bed
    Bed = 11,
    /// Set data of a jigsaw block
    Jigsaw,
    /// Set items in a campfire
    Campfire,
    /// Beehive information
    Beehive,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct MultiBlockChange<'a> {
    /// Chunk section coordinate (encoded chunk x and z with each 22 bits, and section y with 20 bits, from left to right).
    ///
    /// Use [MultiBlockChange::decode_chunk_section_position] and [MultiBlockChange::encode_chunk_section_position] to work with it.
    pub chunk_section_position: u64,
    /// Always inverse the preceding Update Light packet's "Trust Edges" bool
    pub inverse_trust_edges: bool,
    /// Each entry is composed of the block id, shifted right by 12, and the relative block position in the chunk section (4 bits for x, z, and y, from left to right).
    ///
    /// Use [MultiBlockChange::decode_block] and [MultiBlockChange::encode_block] to work with it.
    pub blocks: Array<'a, VarLong, VarInt>,
}

impl<'a> MultiBlockChange<'a> {
    /// Takes the position of the chunk (block coordinate divided by 16 and rounded down).
    pub fn encode_chunk_section_position(x: i32, y: i32, z: i32) -> Result<u64, &'static str> {
        use std::mem::transmute;
        let (x, y, z): (u32, u32, u32) = unsafe { (transmute(x), transmute(y), transmute(z)) };

        if x > 0x3FFFFF || y > 0xFFFFF || z > 0x3FFFFF {
            return Err(
                "Unable to encode block: found a value out of range for the protocol types.",
            );
        }

        Ok((x as u64 & 0x3FFFFF) << 42 | (y as u64 & 0xFFFFF) | (z as u64 & 0x3FFFFF) << 20)
    }

    /// Returns the position of the chunk (block coordinate divided by 16 and rounded down).
    pub fn decode_chunk_section_position(chunk_section_position: u64) -> (i32, i32, i32) {
        let x = (chunk_section_position >> 42) as u32;
        let y = (chunk_section_position << 44 >> 44) as u32;
        let z = (chunk_section_position << 22 >> 42) as u32;

        use std::mem::transmute;
        unsafe { (transmute(x), transmute(y), transmute(z)) }
    }

    /// Takes the position of the block relatively to the position of the chunk passed in `chunk_section_position`.
    ///
    /// ```ignore
    /// // get the relative X coordinate
    /// let chunk_x = (x / 16.0).floor();
    /// let relative_x = x - chunk_x * 16;
    /// ```
    pub fn encode_block(id: i32, x: u8, y: u8, z: u8) -> Result<u64, &'static str> {
        if x > 0xF || y > 0xF || z > 0xF || id < 0 {
            return Err(
                "Unable to encode block: found a value out of range for the protocol types.",
            );
        }

        Ok((id as u64) << 12 | ((x as u64) << 8 | (y as u64) << 4 | z as u64))
    }

    /// Returns the position of the block relatively to the position of the chunk passed in `chunk_section_position`.
    ///
    /// ```ignore
    /// // get the absolute X coordinate
    /// let x = chunk_x * 16 + relative_x;
    /// ```
    pub fn decode_block(block: u64) -> (i32, u8, u8, u8) {
        let id: i32 = unsafe { std::mem::transmute((block >> 12) as u32) };
        let x = (block << 52 >> 60) as u8;
        let z = (block << 56 >> 60) as u8;
        let y = (block << 60 >> 60) as u8;
        (id, x, z, y)
    }
}
