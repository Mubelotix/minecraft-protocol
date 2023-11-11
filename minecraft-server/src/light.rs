use std::{collections::BinaryHeap, cmp::Ordering};

use crate::prelude::*;


#[derive(Debug, Clone)]
struct SectionLightData(Vec<u8>);

impl SectionLightData {
    pub fn new() -> SectionLightData {
        SectionLightData(vec![0; 2048])
    }

    /// Get the light level at the given position.
    pub fn get(&self, postion: BlockPositionInChunk) -> u8 {
        let (x, y, z) = (postion.bx as usize, postion.by as usize, postion.bz as usize);
        let index = (y << 4) | (z << 4) | x;
        let byte_index = index >> 1;

        assert!(byte_index < 2048);

        if index & 1 == 0 {
            self.0[byte_index] & 0x0F
        } else {
            (self.0[byte_index] & 0xF0) >> 4
        }
    }

    /// Set the light level at the given position.
    pub fn set(&mut self, postion: BlockPositionInChunk, level: u8) {
        let (x, y, z) = (postion.bx as usize, postion.by as usize, postion.bz as usize);
        let index = (y << 4) | (z << 4) | x;
        let byte_index = index >> 1;

        if index & 1 == 0 {
            self.0[byte_index] = (self.0[byte_index] & 0xF0) | (level & 0x0F);
        } else {
            self.0[byte_index] = (self.0[byte_index] & 0x0F) | ((level & 0x0F) << 4);
        }
    }
    
    /// Set the light level at the given slice to the given level.
    pub(super) fn set_slice(&mut self, slice: u8 , level: u8) {
        let slice_index = (slice as usize) << 4;
        let level_byte = level << 4 | level;
        for z in 0..16 {
            let z_index = slice_index | (z << 4);
            for x in 0..8 {
                let index = z_index | x;
                self.0[index] = level_byte;
            }
        }
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

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, 15);
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }), 15);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, 0);
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }), 0);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }, 1);
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }), 1);

        data.set(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }, 15);
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }), 15);

        data.set(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }, 1);
        assert_eq!(data.get(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }), 1);

    }
}
