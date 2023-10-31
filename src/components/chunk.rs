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


#[derive(Debug)]
pub enum PalettedData<const LBITS: u8, const HBITS: u8, const DBITS: u8, const TRUNC: usize> {
    Paletted {
        palette: Vec<u32>,
        indexed: Vec<u8>,
    },
    Single {
        value: u32,
    },
    Raw {
        values: Vec<u32>,
    }
}

impl<'a, const LBITS: u8, const HBITS: u8, const DBITS: u8, const TRUNC: usize> MinecraftPacketPart<'a> for PalettedData<LBITS, HBITS, DBITS, TRUNC> {
    fn deserialize_minecraft_packet_part(input: &'a [u8]) -> Result<(Self, &'a [u8]), &'static str> {
        let (mut bits_per_entry, new_input) = u8::deserialize_minecraft_packet_part(input)?;

        Ok(match bits_per_entry {
            0 => {
                let (value, new_input) = VarInt::deserialize_minecraft_packet_part(new_input)?;
                let (longs, new_input) = <Array<u64, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                if !longs.items.is_empty() {
                    return Err("non-empty longs array for 0 bits per entry");
                }
                (PalettedData::Single { value: value.0 as u32 }, new_input)
            },
            _ if bits_per_entry<=HBITS => {
                if bits_per_entry<LBITS {
                    bits_per_entry = LBITS;
                }
                let entries_per_long = 64 / bits_per_entry;
                let mut base_mask = 0;
                for _ in 0..bits_per_entry {
                    base_mask <<= 1;
                    base_mask += 1;
                }

                let (palette, new_input) = <Array<VarInt, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                let palette: Vec<u32> = palette.items.into_iter().map(|id| id.0 as u32).collect();

                let (longs, new_input) = <Array<u64, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                let mut indexed = Vec::new();
                for long in longs.items.into_iter() {
                    let mut mask = base_mask;
                    for i in 0..entries_per_long {
                        let index = ((long & mask) >> (i * bits_per_entry)) as u8;
                        indexed.push(index);
                        mask <<= bits_per_entry;
                    }
                }
                indexed.truncate(TRUNC);

                (PalettedData::Paletted { palette, indexed }, new_input)
            },
            _ => {
                bits_per_entry = DBITS;
                let entries_per_long = 64 / bits_per_entry;
                let mut base_mask = 0;
                for _ in 0..bits_per_entry {
                    base_mask <<= 1;
                    base_mask += 1;
                }

                let (longs, new_input) = <Array<u64, VarInt>>::deserialize_minecraft_packet_part(new_input)?;
                let mut values = Vec::new();
                for long in longs.items.into_iter() {
                    let mut mask = base_mask;
                    for i in 0..entries_per_long {
                        let value = ((long & mask) >> (i * bits_per_entry)) as u32;
                        values.push(value);
                        mask <<= bits_per_entry;
                    }
                }
                values.truncate(TRUNC);

                (PalettedData::Raw { values }, new_input)
            }
        })
    }

    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        match self {
            PalettedData::Single { value } => {
                0u8.serialize_minecraft_packet_part(output)?;
                VarInt::from(value as usize).serialize_minecraft_packet_part(output)?;
                VarInt::from(0).serialize_minecraft_packet_part(output)?;
                Ok(())
            },
            PalettedData::Paletted { palette, indexed } => {
                let mut bits_per_entry = 64 - (palette.len()-1).leading_zeros();

                // If the palette is too big, drop it and use raw data
                if bits_per_entry > HBITS as u32 {
                    let mut values = Vec::new();
                    for index in indexed.iter() {
                        values.push(palette[*index as usize]);
                    }
                    return PalettedData::<LBITS, HBITS, DBITS, TRUNC>::Raw{values}.serialize_minecraft_packet_part(output);
                }

                (bits_per_entry as u8).serialize_minecraft_packet_part(output)?;
                if bits_per_entry < LBITS as u32 {
                    bits_per_entry = LBITS as u32;
                }
                let entries_per_long = 64 / bits_per_entry;

                let palette: Vec<VarInt> = palette.into_iter().map(|id| VarInt::from(id as usize)).collect::<Vec<VarInt>>();
                let palette: Array<VarInt, VarInt> = Array::from(palette);
                palette.serialize_minecraft_packet_part(output)?;

                let mut longs: Vec<u64> = Vec::new();
                for entries in indexed.chunks(entries_per_long as usize) {
                    let mut long = 0;
                    for entry in entries.iter() {
                        long <<= bits_per_entry;
                        long += *entry as u64;
                    }
                    longs.push(long);
                }
                let longs: Array<u64, VarInt> = Array::from(longs);
                longs.serialize_minecraft_packet_part(output)?;

                Ok(())
            },
            PalettedData::Raw { values } => {
                DBITS.serialize_minecraft_packet_part(output)?;
                let bits_per_entry = DBITS;
                let entries_per_long = 64 / bits_per_entry;

                let mut longs: Vec<u64> = Vec::new();
                for entries in values.chunks(entries_per_long as usize) {
                    let mut long = 0;
                    for entry in entries.iter() {
                        long <<= bits_per_entry;
                        long += *entry as u64;
                    }
                    longs.push(long);
                }
                let longs: Array<u64, VarInt> = Array::from(longs);
                longs.serialize_minecraft_packet_part(output)?;

                Ok(())
            },
        }
    }
}

/// A [chunk section](ChunkSection) is a 16×24×16 collection of blocks (chunk sections are cubic).
/// A [chunk column](ChunkData) is a 16×384×16 collection of blocks, and is what most players think of when they hear the term "chunk".
/// However, these are not the smallest unit data is stored in in the game; [chunk columns](ChunkData) are actually 16 [chunk sections](ChunkSection) aligned vertically.
#[derive(Debug, MinecraftPacketPart)]
pub struct Chunk {
    block_count: i16,
    blocks: PalettedData<4, 8, 15, {16*16*16}>,
    biomes: PalettedData<0, 3, 6, {4*4*4}>,
}

impl Chunk {
    /// Deserialize chunk sections from data in a chunk packet
    pub fn deserialize_from_data(input: &[u8]) -> Result<Vec<Chunk>, &'static str> {
        let chunk_count = (-64..320).len() / 16;
        let (chunks, input) = Chunk::deserialize_n(input, chunk_count)?;

        if !input.is_empty() {
            return Err("trailing data not parsed");
        }

        Ok(chunks)
    }

    pub fn serialize_chunks(chunks: Vec<Chunk>) -> Result<Vec<u8>, &'static str> {
        let mut output = Vec::new();

        let chunk_count = (-64..320).len() / 16;
        if chunks.len() != chunk_count {
            return Err("invalid chunk count");
        }
        for chunk in chunks {
            chunk.serialize_minecraft_packet_part(&mut output)?;
        }

        Ok(output)
    }
}

#[cfg(test)]
#[test]
fn test() {
    //let chunk_data = &include_bytes!("../../test_data/chunk_-10_-1.dump")[..];

    //let chunks = Chunk::deserialize_from_data(chunk_data).unwrap();
    //println!("{chunks:?}");

    // let chunk_data = &include_bytes!("../../test_data/chunk_-10_-1.dump")[2..];
    // let chunks = ChunkData::deserialize_minecraft_packet_part(chunk_data).unwrap();
    // println!("{chunks:?}");

    /*let packet_data: Vec<u8> = include!("../../test_data/chunk2-2.dump").trim().split(",").map(|v| match v.parse::<u8>() {
        Ok(v) => v,
        Err(e) => panic!("invalid {:?}", v)
    }).collect();
    std::fs::write("test_data/chunk2-2.dump", packet_data.clone());*/
    let packet_data = &include_bytes!("../../test_data/chunk2.dump")[..];
    // let (packet, rest) = ChunkData::deserialize_minecraft_packet_part(&packet_data).unwrap();
    // assert!(rest.is_empty());
    // let chunk_data = packet.data.items.as_slice();
    let chunks = Chunk::deserialize_from_data(packet_data).unwrap();
    let data2 = Chunk::serialize_chunks(chunks).unwrap();
    std::fs::write("test_data/chunk3.dump", data2.clone());
    assert_eq!(packet_data, data2.as_slice());
    
    //println!("{:?}", chunk_data_deserialized);
}
