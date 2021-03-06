use crate::{nbt::NbtTag, *};

/// A complex data structure including block data and optionally entities of a chunk.
///
/// Note that the Notchian client requires an [ClientboundPacket::UpdateViewPosition](crate::packets::play_clientbound::ClientboundPacket::UpdateViewPosition) packet when it crosses a chunk border, otherwise it'll only display `render distance + 2` chunks around the chunk it spawned in.
#[derive(Debug)]
pub struct ChunkData<'a> {
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_x: i32,
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_z: i32,
    /// BitSet with bits (world height in blocks / 16) set to 1 for every 16×16×16 chunk section whose data is included in Data.
    /// The least significant bit represents the chunk section at the bottom of the chunk column (from the lowest y to 15 blocks above).
    pub primary_bit_mask: Array<'a, u64, VarInt>,
    /// Compound containing one long array named `MOTION_BLOCKING`, which is a heightmap for the highest solid block at each position in the chunk (as a compacted long array with 256 entries at 9 bits per entry totaling 36 longs).
    /// The Notchian server also adds a `WORLD_SURFACE` long array, the purpose of which is unknown, but it's not required for the chunk to be accepted.
    pub heightmaps: NbtTag,
    /// 1024 biome IDs, ordered by x then z then y, in 4×4×4 blocks.
    /// Biomes cannot be changed unless a chunk is re-sent.
    /// The structure is an array of 1024 integers, each representing a [Biome ID](http://minecraft.gamepedia.com/Biome/ID) (it is recommended that "Void" is used if there is no set biome - its default id is 127). The array is ordered by x then z then y, in 4×4×4 blocks. The array is indexed by `((y >> 2) & 63) << 4 | ((z >> 2) & 3) << 2 | ((x >> 2) & 3)`.
    pub biomes: Array<'a, VarInt, VarInt>,
    /// The data section of the packet contains most of the useful data for the chunk.
    /// The number of elements in the array is equal to the number of bits set in [ChunkData::primary_bit_mask].
    /// Sections are sent bottom-to-top, i.e. the first section, if sent, extends from Y=0 to Y=15.
    ///
    /// **Use [ChunkData::deserialize_chunk_sections] to get ready to use [ChunkSection]s.**
    pub data: &'a [u8],
    /// All block entities in the chunk.
    /// Use the x, y, and z tags in the NBT to determine their positions.
    /// Sending entities is not required; it is still legal to send them with [ClientboundPacket::UpdateBlockEntity] later.
    pub entities: Array<'a, NbtTag, VarInt>,
}

impl<'a> MinecraftPacketPart<'a> for ChunkData<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.chunk_x.serialize_minecraft_packet_part(output)?;
        self.chunk_z.serialize_minecraft_packet_part(output)?;
        self.primary_bit_mask
            .serialize_minecraft_packet_part(output)?;
        self.heightmaps.serialize_minecraft_packet_part(output)?;
        self.biomes.serialize_minecraft_packet_part(output)?;
        VarInt(self.data.len() as i32).serialize_minecraft_packet_part(output)?;
        output.extend_from_slice(self.data);
        self.entities.serialize_minecraft_packet_part(output)?;
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (chunk_x, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (chunk_z, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (primary_bit_mask, input) =
            MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (heightmaps, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (biomes, input) =
                    <Array<'a, VarInt, VarInt>>::deserialize_minecraft_packet_part(input)?;
        let (data_len, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        let data_len = std::cmp::max(data_len.0, 0) as usize;
        let (data, input) = input.split_at(data_len);
        let (entities, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        Ok((
            ChunkData {
                chunk_x,
                chunk_z,
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

/// A [chunk section](ChunkSection) is a 16×16×16 collection of blocks (chunk sections are cubic).
/// A [chunk column](ChunkData) is a 16×256×16 collection of blocks, and is what most players think of when they hear the term "chunk".
/// However, these are not the smallest unit data is stored in in the game; [chunk columns](ChunkData) are actually 16 [chunk sections](ChunkSection) aligned vertically.
#[derive(Debug)]
pub struct ChunkSection {
    /// Number of non-air blocks present in the chunk section, for lighting purposes.
    /// "Non-air" is defined as any block other than air, cave air, and void air (in particular, note that fluids such as water are still counted).
    pub block_count: i16,
    /// Chunk sections often contains a palette (for compression).
    /// This is a great way to find if a particular block is present in the section without iterating trought all blocks.
    pub palette: Option<Vec<u32>>,
    /// Blocks stored as `block state IDs`.
    /// Blocks with increasing x coordinates, within rows of increasing z coordinates, within layers of increasing y coordinates.
    /// Use [Block::from_state_id](crate::ids::blocks::Block::from_state_id) to get the corresponding [Block](crate::ids::blocks::Block).
    pub blocks: Vec<u32>,
}

impl<'a> ChunkData<'a> {
    /// Deserialize chunk sections from a chunk data packet.
    #[allow(clippy::needless_range_loop)]
    pub fn deserialize_chunk_sections(
        &mut self,
    ) -> Result<[Option<ChunkSection>; 16], &'static str> {
        let mut input = self.data;
        if self.primary_bit_mask.items.len() != 1 {
            return Err("ChunkData::deserialize_chunk_sections: primary_bit_mask.items.len() != 1");
        }
        let primary_bit_mask: u64 = unsafe {
            // We don't care the type since we only want to check the bits
            *self.primary_bit_mask.items.get_unchecked(0)
        };

        let mut chunk_sections: [Option<ChunkSection>; 16] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        let mut mask = 0b1;
        for y in 0..16 {
            chunk_sections[y] = if primary_bit_mask & mask != 0 {                
                let (block_count, new_input) = i16::deserialize_minecraft_packet_part(input)?;
                let (mut bits_per_block, new_input) =
                    u8::deserialize_minecraft_packet_part(new_input)?;
                if bits_per_block < 4 {
                    bits_per_block = 4;
                }
                let mut blocks = Vec::new();

                if bits_per_block <= 8 {
                    let (palette_lenght, mut new_input) =
                        VarInt::deserialize_minecraft_packet_part(new_input)?;
                    let palette_lenght = if palette_lenght.0 < 0 {
                        0usize
                    } else {
                        palette_lenght.0 as usize
                    };
                    let mut palette = Vec::with_capacity(10);
                    for _ in 0..palette_lenght {
                        let (palette_item, new_new_input) =
                            VarInt::deserialize_minecraft_packet_part(new_input)?;
                        palette.push(std::cmp::max(palette_item.0, 0) as u32);
                        new_input = new_new_input;
                    }
                    let (longs, new_input) =
                        <Array<u64, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                    let blocks_per_long = match bits_per_block {
                        4 => 16,
                        5 => 12,
                        6 => 10,
                        7 => 9,
                        8 => 8,
                        _ => unreachable!(),
                    };

                    for long in longs.items {
                        let mut mask = match bits_per_block {
                            4 => 0xF,
                            5 => 0b11111,
                            6 => 0b111111,
                            7 => 0b1111111,
                            8 => 0xFF,
                            _ => unreachable!(),
                        };
                        for i in 0..blocks_per_long {
                            let block_index = ((long & mask) >> (i * bits_per_block)) as usize;
                            let block = *palette.get(block_index).unwrap_or(&0);
                            blocks.push(block);
                            mask <<= bits_per_block;
                        }
                    }

                    input = new_input;
                    Some(ChunkSection {
                        block_count,
                        palette: Some(palette),
                        blocks,
                    })
                } else {
                    let (longs, new_input) =
                        <Array<u64, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                    let blocks_per_long = (64.0 / bits_per_block as f32).floor() as u8;

                    for long in longs.items {
                        let mut mask = match bits_per_block {
                            9 => 0b1_1111_1111,
                            10 => 0b11_1111_1111,
                            11 => 0b111_1111_1111,
                            12 => 0xFFF,
                            13 => 0b1_1111_1111_1111,
                            14 => 0b11_1111_1111_1111,
                            _ => return Err("Unsupported bits_per_block"),
                        };
                        for i in 0..blocks_per_long {
                            let block = ((long & mask) >> (i * bits_per_block)) as u32;
                            blocks.push(block);
                            mask <<= bits_per_block;
                        }
                    }

                    input = new_input;
                    Some(ChunkSection {
                        block_count,
                        palette: None,
                        blocks,
                    })
                }
            } else {
                None
            };
            mask <<= 1;
        }

        Ok(chunk_sections)
    }
}

#[cfg(test)]
#[test]
fn test() {
    let chunk_data = &include_bytes!("../../test_data/chunk.mc_packet")[1..];

    let mut chunk_data_deserialized = ChunkData::deserialize_uncompressed_minecraft_packet(chunk_data).unwrap();
    let _blocks = chunk_data_deserialized.deserialize_chunk_sections().unwrap();
    
    //println!("{:?}", chunk_data_deserialized);
}
