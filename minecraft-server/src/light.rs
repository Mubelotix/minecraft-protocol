use std::{collections::BinaryHeap, ops::AddAssign};

use minecraft_protocol::ids::blocks::Block;

use crate::{prelude::*, position};


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
    pub fn set_region(&mut self, from_y: usize, to_y: usize, level: u8) -> Result<(), ()> {
        if level > self.level {
            return Err(());
        }

        // Get the range of sections to set.
        let first_section = from_y.div_euclid(16);
        let first_secion_offset = from_y.rem_euclid(16);

        let last_section = to_y.div_euclid(16);
        let last_section_offset = to_y.rem_euclid(16);

        println!("Setting sky light from {} to {} in sections {} to {}", from_y, to_y, first_section, last_section);
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

            // Update the mask
            let mask = 1 << section;
            if self.level > 0 {
                self.empty_sky_light_mask |= mask;
            } else {
                self.empty_sky_light_mask &= !mask;
            }
        }

        Ok(())
    }

    pub(super) fn get_level(&self, position: LightPositionInChunkColumn) -> Result<u8, ()> {
        let section = position.y.div_euclid(16);
        self.sky_light_arrays[section.max(0)].get(position.in_chunk())
    }

    pub(super) fn set_level(&mut self, position: LightPositionInChunkColumn, level: u8) -> Result<(), ()> {
        let section = position.y.div_euclid(16);
        self.sky_light_arrays[section.max(0)].set(position.in_chunk(), level)
    }
}

pub(super) struct Light {
    sky_light: SkyLight,
}

impl Light {
    pub fn new() -> Self {
        // TODO: Make this configurable with the world.
        Self {
            sky_light: SkyLight {
                level: 15,
                sky_light_arrays: vec![SectionLightData::new(); 24+2],
                sky_light_mask: 0,
                empty_sky_light_mask: !0,
                zero_chunk_index: 4, // We start at y=-64, and we have a chunk under that. 
            },
        }
    }
}

#[derive(Debug, Clone)]
struct LightPositionInChunkColumn {
    pub bx: u8,
    pub y: usize,
    pub bz: u8,
}

impl LightPositionInChunkColumn {
    pub fn in_chunk(&self) -> BlockPositionInChunk {
        BlockPositionInChunk {
            bx: self.bx,
            by: self.y.rem_euclid(16) as u8,
            bz: self.bz,
        }
    }
}

impl PartialEq for LightPositionInChunkColumn {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl From<LightPositionInChunkColumn> for BlockPositionInChunkColumn {
    fn from(val: LightPositionInChunkColumn) -> Self {
        BlockPositionInChunkColumn {
            bx: val.bx,
            y: val.y as i32 - 64 - 16, // TODO: Use the world config
            bz: val.bz,
        }
    }
}

impl From<BlockPositionInChunkColumn> for LightPositionInChunkColumn {
    fn from(val: BlockPositionInChunkColumn) -> Self {
        LightPositionInChunkColumn {
            bx: val.bx,
            y: (val.y + 64 + 16) as usize, //-TODO: Use the world config
            bz: val.bz,
        }
    }   
}

impl AddAssign<usize> for LightPositionInChunkColumn {
    fn add_assign(&mut self, rhs: usize) {
        self.y += rhs;
    }
}

impl std::cmp::Eq for LightPositionInChunkColumn {}

impl std::cmp::PartialOrd for LightPositionInChunkColumn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.y.cmp(&other.y))
    }
}

impl std::cmp::Ord for LightPositionInChunkColumn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y)
    }
}

impl ChunkColumn {
    pub(super) fn init_light(&mut self) -> Result<(), ()> {
        self.propagate_sky_light_inside()?;
        Ok(())
    }

    fn propagate_sky_light_inside(&mut self) -> Result<(), ()> {
        // Set all highest blocks to the highest block
        let highest_blocks = self.get_highest_block();

        let max_y = self.light.sky_light.sky_light_arrays.len() * 16 - 1;
        self.light.sky_light.set_region(highest_blocks as usize + 16, max_y, self.light.sky_light.level)?;
        let mut to_explore: BinaryHeap<LightPositionInChunkColumn> = BinaryHeap::new();
        
        
        // Add all highest blocks to the queue
        for x in 0..16 {
            for z in 0..16 {
                let position = LightPositionInChunkColumn {
                    bx: x,
                    y: self.get_hiest_block_at(&BlockPositionInChunkColumn { bx: x, y: 0, bz: z }) as usize + 16 + 1,
                    bz: z,
                };
                to_explore.push(position);
            } 
        }

        self.explore_sky_light_from_heap(&mut to_explore).map_err(|_| error!("Error while updating light")).unwrap();

        Ok(())
    }

    fn explore_sky_light_from_heap(&mut self, to_explore: &mut BinaryHeap<LightPositionInChunkColumn>) -> Result<(), ()> {
        while let Some(position) = to_explore.pop() {
            let mut neighbors = Vec::new();
            let is_inside = self.get_hiest_block_at(&position.clone().into()) > position.y as u16;
            let my_level = self.light.sky_light.get_level(position.clone())?;
            if position.bx > 0 {
                neighbors.push(LightPositionInChunkColumn { bx: position.bx - 1, y: position.y, bz: position.bz });
            }
            if position.bx < 15 {
                neighbors.push(LightPositionInChunkColumn { bx: position.bx + 1, y: position.y, bz: position.bz });
            }
            if position.bz > 0 {
                neighbors.push(LightPositionInChunkColumn { bx: position.bx, y: position.y, bz: position.bz - 1 });
            }
            if position.bz < 15 {
                neighbors.push(LightPositionInChunkColumn { bx: position.bx, y: position.y, bz: position.bz + 1 });
            }
            if position.y > 0 {
                neighbors.push(LightPositionInChunkColumn { bx: position.bx, y: position.y - 1, bz: position.bz });
            }
            if position.y < ((self.light.sky_light.sky_light_arrays.len() - 1) * 16) + 1 { // No block can be higher so no block can affect the light level 
                neighbors.push(LightPositionInChunkColumn { bx: position.bx, y: position.y + 1, bz: position.bz });
            }

            for neighbor in neighbors {
                let neighbor_level = self.light.sky_light.get_level(neighbor.clone())?;

                let block = Block::from(self.get_block(neighbor.clone().into())); 
                if block.is_transparent() 
                    && (neighbor_level < my_level.saturating_sub(1)) 
                {
                    let highest_block = self.get_hiest_block_at(&neighbor.clone().into()) + 16;
                    let is_inside = highest_block > neighbor.y as u16 + 1;
                    to_explore.push(neighbor.clone());
                    let new_level = if is_inside { my_level - 1 } else { self.light.sky_light.level };
                    self.light.sky_light.set_level(neighbor, new_level)?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn update_light_at(&mut self, position: BlockPositionInChunkColumn) {
        let position = LightPositionInChunkColumn::from(position);
        let (bx, y, bz) = (position.bx, position.y, position.bz);
        
        let mut to_explore: BinaryHeap<LightPositionInChunkColumn> = BinaryHeap::from(vec![
            LightPositionInChunkColumn { bx, y: y + 1, bz },
            LightPositionInChunkColumn { bx: (bx + 1) % 16, y, bz },
            LightPositionInChunkColumn { bx: bx.saturating_sub(1), y, bz },
            LightPositionInChunkColumn { bx, y, bz: (bz + 1) % 16 },
            LightPositionInChunkColumn { bx, y, bz: bz.saturating_sub(1)},
        ]);

        self.explore_sky_light_from_heap(&mut to_explore).map_err(|_| error!("Error while updating light")).unwrap();
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

        sky_light.set_region(1, 33, 15).unwrap();
        
        // Test in
        assert_eq!(sky_light.sky_light_arrays[0].get(BlockPositionInChunk { bx: 0, by: 1, bz: 7 }).unwrap(), 15);
        assert_eq!(sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 1, by: 15, bz: 8 }).unwrap(), 15);
        assert_eq!(sky_light.sky_light_arrays[2].get(BlockPositionInChunk { bx: 3, by: 0, bz: 0 }).unwrap(), 15);

        // Test out
        assert_eq!(sky_light.sky_light_arrays[0].get(BlockPositionInChunk { bx: 4, by: 0, bz: 2 }).unwrap(), 0);
        assert_eq!(sky_light.sky_light_arrays[3].get(BlockPositionInChunk { bx: 0, by: 14, bz: 9 }).unwrap(), 0);
        assert_eq!(sky_light.sky_light_arrays[4].get(BlockPositionInChunk { bx: 9, by: 0, bz: 10 }).unwrap(), 0);
    }

    #[test]
    fn test_sky_light_flat_chunk() {
        let mut flat_chunk = ChunkColumn::flat();

        // Check that the sky light is equal to the light level above the grass and on the top of the world.
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[0].get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), 0);
                assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[4].get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), 15);
                assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[25].get(BlockPositionInChunk { bx: x, by: 15, bz: z }).unwrap(), 15);
            }
        }
        
        // Break the grass block and check that the sky light is correct.
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 15, bz: 0 }).unwrap(), 0);
        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -49, bz: 0 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 15, bz: 0 }).unwrap(), 15);
        
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 0 }).unwrap(), 0);
        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 0 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 0 }).unwrap(), 15);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 1 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 1 }).unwrap(), 14);
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 0, bz: 10 }).unwrap(), 0);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 2 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 2 }).unwrap(), 13);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -51, bz: 2 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.sky_light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 13, bz: 2 }).unwrap(), 12);

    }
}
