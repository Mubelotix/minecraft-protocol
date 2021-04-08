use crate::{*, nbt::NbtTag};

/// A complex data structure including block data and optionally entities of a chunk.
/// 
/// Note that the Notchian client requires an Update View Position packet when it crosses a chunk border, otherwise it'll only display render distance + 2 chunks around the chunk it spawned in.
#[derive(Debug)]
pub struct ChunkData<'a> {
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_x: i32,
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_y: i32,
    /// Bitmask with bits set to 1 for every 16×16×16 chunk section whose data is included in Data.
    /// The least significant bit represents the chunk section at the bottom of the chunk column (from y=0 to y=15).
    pub primary_bit_mask: VarInt,
    /// Compound containing one long array named `MOTION_BLOCKING`, which is a heightmap for the highest solid block at each position in the chunk (as a compacted long array with 256 entries at 9 bits per entry totaling 36 longs). The Notchian server also adds a `WORLD_SURFACE` long array, the purpose of which is unknown, but it's not required for the chunk to be accepted.
    pub heightmaps: NbtTag<'a>,
    /// 1024 biome IDs, ordered by x then z then y, in 4×4×4 blocks.
    /// Biomes cannot be changed unless a chunk is re-sent.
    /// The structure is an array of 1024 integers, each representing a [Biome ID](http://minecraft.gamepedia.com/Biome/ID) (it is recommended that "Void" is used if there is no set biome - its default id is 127). The array is ordered by x then z then y, in 4×4×4 blocks. The array is indexed by `((y >> 2) & 63) << 4 | ((z >> 2) & 3) << 2 | ((x >> 2) & 3)`.
    pub biomes: Option<Array<'a, VarInt, VarInt>>,
    /// The data section of the packet contains most of the useful data for the chunk.
    /// The number of elements in the array is equal to the number of bits set in [ChunkData::primary_bit_mask].
    /// Sections are sent bottom-to-top, i.e. the first section, if sent, extends from Y=0 to Y=15.
    pub data: Array<'a, i8, VarInt>,
    /// All block entities in the chunk.
    /// Use the x, y, and z tags in the NBT to determine their positions.
    /// Sending entities is not required; it is still legal to send them with [ClientBoundPacket::UpdateBlockEntity] later.
    pub entities: Array<'a, NbtTag<'a>, VarInt>,
}

impl<'a> MinecraftPacketPart<'a> for ChunkData<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.chunk_x.serialize_minecraft_packet_part(output)?;
        self.chunk_y.serialize_minecraft_packet_part(output)?;
        self.biomes.is_some().serialize_minecraft_packet_part(output)?;
        self.primary_bit_mask
            .serialize_minecraft_packet_part(output)?;
        self.heightmaps.serialize_minecraft_packet_part(output)?;
        if let Some(biomes) = self.biomes {
            biomes.serialize_minecraft_packet_part(output)?;
        }
        self.data.serialize_minecraft_packet_part(output)?;
        self.entities.serialize_minecraft_packet_part(output)?;
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (chunk_x, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (chunk_y, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (full_chunk, input) =
            MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (primary_bit_mask, input) =
            MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (heightmaps, input) =
            MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (biomes, input) = match full_chunk {
            false => (None, input),
            true => {
                let (biomes, input) = <Array<'a, VarInt, VarInt>>::deserialize_minecraft_packet_part(input)?;
                (Some(biomes), input)
            }
        };
        let (data, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (entities, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        Ok((
            ChunkData {
                chunk_x,
                chunk_y,
                primary_bit_mask,
                heightmaps,
                biomes,
                data,
                entities,
            },
            input,
        ))
    }
}
