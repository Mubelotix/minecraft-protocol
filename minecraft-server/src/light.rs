use std::{collections::BinaryHeap, cmp::Ordering};

use crate::prelude::*;


#[derive(Debug, Clone)]
struct SectionLightData(Vec<u8>);

impl SectionLightData {
    pub fn new() -> SectionLightData {
        SectionLightData(vec![0; 2048])
    }

    pub fn set_with(&mut self, level: u8) {
        let level = level << 4 | level;
        self.0.iter_mut().for_each(|v| *v = level);
    }
    

    /// Get the light level at the given position.
    pub fn get(&self, postion: BlockPositionInChunk) -> Result<u8, ()> {
        let (x, y, z) = (postion.bx as usize, postion.by as usize, postion.bz as usize);
        let index = (y << 8) | (z << 4) | x;
        let byte_index = index >> 1;

        if byte_index >= 2048 {
            return Err(());
        }

        if index & 1 == 0 {
            Ok(self.0[byte_index] & 0x0F)
        } else {
            Ok((self.0[byte_index] & 0xF0) >> 4)
        }
    }

    /// Set the light level at the given position.
    pub fn set(&mut self, postion: BlockPositionInChunk, level: u8) -> Result<(), ()> {
        if level > 15 {
            return Err(());
        }

        let (x, y, z) = (postion.bx as usize, postion.by as usize, postion.bz as usize);
        let index = (y << 8) | (z << 4) | x;
        let byte_index = index >> 1;
        
        if byte_index >= 2048 {
            return Err(());
        }

        if index & 1 == 0 {
            self.0[byte_index] = (self.0[byte_index] & 0xF0) | (level & 0x0F);
        } else {
            self.0[byte_index] = (self.0[byte_index] & 0x0F) | ((level & 0x0F) << 4);
        }

        Ok(())
    }
    
    /// Set the light level at the given layer to the given level.
    pub(super) fn set_layer(&mut self, layer: u8 , level: u8) -> Result<(), ()> {
        if level > 15 {
            return Err(());
        }
        
        if layer > 15 {
            return Err(());
        }

        let level = level << 4 | level;
        let layer = layer as usize;
        
        // Because a layer is 16x16 blocks, we can just set 128 blocks at once and the y coordinate is the most significant bit of the index. 
        for i in layer*128..(layer+1)*128 {
            self.0[i] = level;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SkyLight {
    /// The level of the sky light, 15 is the maximum.
    pub level: u8,
    /// The sky light data for each section.
    pub sky_light_arrays: Vec<SectionLightData>,
    /// The mask of sections that have sky light data.
    pub sky_light_mask: u64,
    /// The mask of sections that don't have sky light data.
    pub empty_sky_light_mask: u64,
    zero_chunk_index: usize,
}

impl SkyLight {
    /// Set the sky light in the given section.
    pub fn set_region(&mut self, from_y: i32, to_y: i32, level: u8) -> Result<(), ()> {
        if level > self.level {
            return Err(());
        }

        // Get the range of sections to set.
        let first_section = (from_y.div_euclid(16) + self.zero_chunk_index as i32).max(0) as usize;
        let first_secion_offset = from_y.rem_euclid(16) as usize;

        let last_section = (to_y.div_euclid(16) + self.zero_chunk_index as i32).max(0) as usize;
        let last_section_offset = to_y.rem_euclid(16) as usize;

        for section in first_section..=last_section {
            if section != first_section && section != last_section {
                // Set the whole section
                self.sky_light_arrays[section].set_with(level);
            } else {
                // Set the part of the section
                let first_offset = if section == first_section { first_secion_offset } else { 0 };
                let last_offset = if section == last_section { last_section_offset } else { 15 };
                for y in first_offset..=last_offset {
                    self.sky_light_arrays[section].set_layer(y as u8, level)?;
                }
            }
        }

        Ok(())
    }
}

pub(super) struct Light {
    sky_light: SkyLight,
}

impl Light {
    pub fn new() -> Self {
        Self {
            sky_light: SkyLight {
                level: 15,
                sky_light_arrays: vec![SectionLightData::new(); 16],
                sky_light_mask: 0,
                empty_sky_light_mask: !0,
                zero_chunk_index: 4, // We start at y=-64, and we have a chunk under that. 
            },
        }
    }
}

struct HeightBasedPosition {
    x: u8,
    y: usize,
    z: u8,
}

impl PartialEq for HeightBasedPosition {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl std::cmp::Eq for HeightBasedPosition {}

impl std::cmp::PartialOrd for HeightBasedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y))
    }
}

impl std::cmp::Ord for HeightBasedPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

impl ChunkColumn {
    pub fn propagate_sky_light_inside(&mut self) {
        // Set all highest blocks to the highest block
        let highest_blocks = self.get_highest_block();

        let n_chunk_with_sky_light =  highest_blocks;
        

        let mut to_explore: BinaryHeap<HeightBasedPosition> = BinaryHeap::new();
        
        
        // Add all highest blocks to the queue
        for x in 0..16 {
            for z in 0..16 {
            } 
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_light_data() {
        let mut data = SectionLightData::new();

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, 15).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }).unwrap(), 15);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, 0).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }).unwrap(), 0);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }).unwrap(), 1);

        data.set(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }, 15).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }).unwrap(), 15);

        data.set(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }).unwrap(), 1);

        data.set(BlockPositionInChunk { bx: 2, by: 0, bz: 0 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 2, by: 0, bz: 0 }).unwrap(), 1);

        // Manual layer
        for z in 0..16 {
            for x in 0..16 {
                data.set(BlockPositionInChunk { bx: x, by: 0, bz: z }, 15).unwrap();
            }
        }

        for z in 0..16 {
            for x in 0..16 {
                assert_eq!(data.get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), 15, "x: {}, z: {}", x, z);
            }
        }

        // Test layer
        data.set_layer(1, 15).unwrap();
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(data.get(BlockPositionInChunk { bx: x, by: 1, bz: z }).unwrap(), 15, "x: {}, z: {}", x, z);
            }
        }
    }

    #[test]
    fn test_set_region() {
        let mut sky_light = SkyLight {
            level: 15,
            sky_light_arrays: vec![SectionLightData::new(); 16+2],
            sky_light_mask: 0,
            empty_sky_light_mask: !0,
            zero_chunk_index: 4, // We start at y=-64, and we have a chunk under that. 
        };

        sky_light.set_region(-1, 16, 15).unwrap();
        
        // Test in
        assert_eq!(sky_light.sky_light_arrays[5].get(BlockPositionInChunk { bx: 0, by: 0, bz: 7 }).unwrap(), 15);
        assert_eq!(sky_light.sky_light_arrays[4].get(BlockPositionInChunk { bx: 1, by: 2, bz: 8 }).unwrap(), 15);
        assert_eq!(sky_light.sky_light_arrays[4].get(BlockPositionInChunk { bx: 3, by: 0, bz: 0 }).unwrap(), 15);

        // Test out
        assert_eq!(sky_light.sky_light_arrays[5].get(BlockPositionInChunk { bx: 4, by: 1, bz: 2 }).unwrap(), 0);
        assert_eq!(sky_light.sky_light_arrays[3].get(BlockPositionInChunk { bx: 0, by: 14, bz: 9 }).unwrap(), 0);
        assert_eq!(sky_light.sky_light_arrays[0].get(BlockPositionInChunk { bx: 9, by: 0, bz: 10 }).unwrap(), 0);

    }
}
