use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
pub struct ChunkBiomeData<'a> {
    /// Chunk coordinate (block coordinate divided by 16, rounded down)
    pub chunk_x: i32, 
    /// Chunk coordinate (block coordinate divided by 16, rounded down)
    pub chunk_z: i32, 
    /// Chunk [data structure](https://wiki.vg/Chunk_Format#Data_structure), with [sections](https://wiki.vg/Chunk_Format#Chunk_Section) containing only the `Biomes` field
    pub data: Array<'a, u8, VarInt>,
}


