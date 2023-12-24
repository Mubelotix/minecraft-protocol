use crate::prelude::*;
use super::*;

const MAX_LIGHT_LEVEL: u8 = 15;

#[derive(Debug, Clone)]
struct SectionLightData(Vec<u8>); // TODO(optimization): Use simd 

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
        if level > MAX_LIGHT_LEVEL {
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
        if level > MAX_LIGHT_LEVEL {
            return Err(());
        }

        if layer > MAX_LIGHT_LEVEL {
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

