use crate::{nbt::NbtTag, *, components::blocks::BlockEntity};

/// A complex data structure including block data and optionally entities of a chunk.
///
/// Note that the Notchian client requires an [ClientboundPacket::UpdateViewPosition](crate::packets::play_clientbound::ClientboundPacket::UpdateViewPosition) packet when it crosses a chunk border, otherwise it'll only display `render distance + 2` chunks around the chunk it spawned in.
#[derive(MinecraftPacketPart, Debug)]
pub struct ChunkData<'a> {
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_x: i32,
    /// Chunk coordinate (block coordinate divided by 16, rounded down).
    pub chunk_z: i32,
    /// Compound containing one long array named `MOTION_BLOCKING`, which is a heightmap for the highest solid block at each position in the chunk (as a compacted long array with 256 entries at 9 bits per entry totaling 36 longs).
    /// The Notchian server also adds a `WORLD_SURFACE` long array, the purpose of which is unknown, but it's not required for the chunk to be accepted.
    pub heightmaps: NbtTag,
    /// The data section of the packet contains most of the useful data for the chunk.
    /// The number of elements in the array is equal to the number of bits set in [ChunkData::primary_bit_mask].
    /// Sections are sent bottom-to-top, i.e. the first section, if sent, extends from Y=0 to Y=15.
    ///
    /// **Use [ChunkData::deserialize_chunk_sections] to get ready to use [ChunkSection]s.**
    pub data: Array<'a, u8, VarInt>,
    pub block_entities: Array<'a, BlockEntity, VarInt>,
    /// BitSet containing bits for each section in the world + 2.
    /// Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Sky Light array below.
    /// The least significant bit is for blocks 16 blocks to 1 block below the min world height (one section below the world), while the most significant bit covers blocks 1 to 16 blocks above the max world height (one section above the world).
    pub sky_light_mask: Array<'a, u64, VarInt>,
    /// BitSet containing bits for each section in the world + 2.
    /// Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Block Light array below.
    /// The order of bits is the same as in Sky Light Mask.
    pub block_light_mask: Array<'a, u64, VarInt>,
    /// BitSet containing bits for each section in the world + 2.
    /// Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Block Light array below.
    /// The order of bits is the same as in Sky Light Mask.
    pub empty_sky_light_mask: Array<'a, u64, VarInt>,
    /// BitSet containing bits for each section in the world + 2.
    /// Each set bit indicates that the corresponding 16×16×16 chunk section has data in the Block Light array below.
    /// The order of bits is the same as in Sky Light Mask.
    pub empty_block_light_mask: Array<'a, u64, VarInt>,
    /// Length should match the number of bits set in Sky Light Mask.
    /// Each entry is an array of 2048 bytes.
    /// There is 1 array for each bit set to true in the sky light mask, starting with the lowest value. Half a byte per light value. Indexed ((y<<8) | (z<<4) | x) / 2
    /// If there's a remainder, masked 0xF0 else 0x0F.
    pub sky_light: Array<'a, Array<'a, u8, VarInt>, VarInt>,
    /// Length should match the number of bits set in Block Light Mask.
    /// Each entry is an array of 2048 bytes.
    /// There is 1 array for each bit set to true in the block light mask, starting with the lowest value. Half a byte per light value. Indexed ((y<<8) | (z<<4) | x) / 2
    /// If there's a remainder, masked 0xF0 else 0x0F.
    pub block_light: Array<'a, Array<'a, u8, VarInt>, VarInt>,
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
        /*let mut input = self.data;
        if self.sky_light_mask.items.len() != 1 {
            return Err("ChunkData::deserialize_chunk_sections: sky_light_mask.items.len() != 1");
        }
        let sky_light_mask: u64 = unsafe {
            // We don't care the type since we only want to check the bits
            *self.sky_light_mask.items.get_unchecked(0)
        };

        let mut chunk_sections: [Option<ChunkSection>; 16] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        let mut mask = 0b1;
        for y in 0..16 {
            chunk_sections[y] = if sky_light_mask & mask != 0 {                
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

        Ok(chunk_sections)*/
        todo!()
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
