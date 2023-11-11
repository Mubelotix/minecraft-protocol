use crate::prelude::*;


#[derive(Debug, Clone)]
pub struct SectionLightData(Vec<u8>);

impl SectionLightData {
    pub fn new() -> SectionLightData {
        SectionLightData(vec![0; 2048])
    }

    /// Get the light level at the given position.
    pub fn get(&self, postion: ChunkPosition) -> u8 {
        let (x, y, z) = (postion.cx as usize, postion.cy as usize, postion.cz as usize);
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
    pub fn set(&mut self, postion: ChunkPosition, level: u8) {
        let (x, y, z) = (postion.cx as usize, postion.cy as usize, postion.cz as usize);
        let index = (y << 4) | (z << 4) | x;
        let byte_index = index >> 1;

        if index & 1 == 0 {
            self.0[byte_index] = (self.0[byte_index] & 0xF0) | (level & 0x0F);
        } else {
            self.0[byte_index] = (self.0[byte_index] & 0x0F) | ((level & 0x0F) << 4);
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkyLight {
    /// The level of the sky light, 15 is the maximum.
    pub level: u8,
    /// The sky light data for each section.
    pub sky_light_arrays: Vec<SectionLightData>,
    /// The mask of sections that have sky light data.
    pub sky_light_mask: u64,
    /// The mask of sections that don't have sky light data.
    pub empty_sky_light_mask: u64,
}

pub struct Light {
    pub sky_light: SkyLight,
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
