use std::collections::BinaryHeap;

use minecraft_protocol::ids::blocks::Block;
use tokio::sync::OwnedRwLockWriteGuard;

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

#[derive(Debug, Clone)]
struct LightSystem {
    /// The level of the sky light, 15 is the maximum.
    pub level: u8,
    /// The sky light data for each section.
    pub light_arrays: Vec<SectionLightData>,
    /// The mask of sections that have sky light data.
    pub light_mask: u64,
    /// The mask of sections that don't have sky light data.
    pub empty_light_mask: u64,
}

impl LightSystem {
    /// Get the light data as an array of arrays.
    fn to_array<'a>(&self) ->  Array<'a, Array<'a, u8, VarInt>, VarInt> {
        let mut sections = Vec::new();
        for (i, section) in self.light_arrays.iter().enumerate() {
            if self.light_mask & (1 << i) != 0 {
                let mut data = Vec::new();
                for byte in section.0.iter() {
                    data.push(*byte);
                }
                sections.push(Array::from(data));
            }
        }
        Array::from(sections)
    }

    /// Get the light mask and the empty light mask as bitsets.
    /// return (light_mask, empty_light_mask)
    fn masks_to_bitset<'a>(&self) -> (BitSet<'a>, BitSet<'a>) {
        let light_mask = BitSet::from(vec![self.light_mask as i64]);
        let empty_light_mask = BitSet::from(vec![self.empty_light_mask as i64]);
        (light_mask, empty_light_mask)
    } 

    /// Get the light data and the light mask and the empty light mask as bitsets.
    /// return (light_data, light_mask, empty_light_mask)
    pub fn get_packet_data<'a>(&self) -> (Array<'a, Array<'a, u8, VarInt>, VarInt>, BitSet<'a>, BitSet<'a>) {
        let data = self.to_array();
        let (light_mask, empty_light_mask) = self.masks_to_bitset();
        (data, light_mask, empty_light_mask)
    }

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


        for section in first_section..=last_section {
            if section != first_section && section != last_section {
                // Set the whole section
                self.light_arrays[section].set_with(level);
            } else {
                // Set the part of the section
                let first_offset = if section == first_section { first_secion_offset } else { 0 };
                let last_offset = if section == last_section { last_section_offset } else { MAX_LIGHT_LEVEL as usize };
                for y in first_offset..=last_offset {
                    self.light_arrays[section].set_layer(y as u8, level)?;
                }
            }

            // Update the mask
            let mask = 1 << section;
            if self.level > 0 {
                self.empty_light_mask &= !mask;
                self.light_mask |= mask;
            } else {
                self.empty_light_mask |= mask;
                self.light_mask &= !mask;
            }
        }

        Ok(())
    }

    pub(super) fn get_level(&self, position: LightPositionInChunkColumn) -> Result<u8, ()> {
        let section = position.y.div_euclid(16);
        self.light_arrays[section.max(0)].get(position.in_chunk())
    }

    pub(super) fn set_level(&mut self, position: LightPositionInChunkColumn, level: u8) -> Result<(), ()> {
        let section = position.y.div_euclid(16);
        // Update the mask
        let mask = 1 << section;
        if self.level > 0 {
            self.empty_light_mask &= !mask;
            self.light_mask |= mask;
        } else {
            // TODO: don't apply this if another block contains the light
            self.empty_light_mask |= mask;
            self.light_mask &= !mask;
        }
        self.light_arrays[section.max(0)].set(position.in_chunk(), level)?;
        Ok(())
    }
}

pub(super) struct Light {
    sky_light: LightSystem,
}

impl Light {
    pub fn new() -> Self {
        // TODO: Make this configurable with the world.
        Self {
            sky_light: LightSystem {
                level: MAX_LIGHT_LEVEL,
                light_arrays: vec![SectionLightData::new(); 24+2],
                light_mask: 0,
                empty_light_mask: !0,
            },
        }
    }

    pub fn get_packet(&self) -> (Array<Array<u8, VarInt>, VarInt>, BitSet, BitSet) {
        self.sky_light.get_packet_data()
    }

    pub fn get_skylight_level(&self, position: LightPositionInChunkColumn) -> u8 {
        self.sky_light.get_level(position).unwrap_or_default()
    }
}

pub struct LightManager {
    world_map: &'static WorldMap,
    current_shard_id: Option<usize>,
    current_shard: Option<OwnedRwLockWriteGuard<HashMap<ChunkColumnPosition, ChunkColumn>>>,
}

impl LightManager {
    fn new(world_map: &'static WorldMap) -> Self {
        Self {
            world_map,
            current_shard: None,
            current_shard_id: None,
        }
    }

    #[instrument(skip(world_map))]
    pub async fn update_light(world_map: &'static WorldMap, block_position: BlockPosition, block: BlockWithState) {
        let mut light_manager = Self::new(world_map);

        light_manager.set_block(block_position, block).await;
    }

    async fn ensure_shard(&mut self, shard_id: usize) {
        if let Some(current_shard_id) = self.current_shard_id  {
            if current_shard_id == shard_id {
                return;
            }
        }
        self.current_shard = Some(self.world_map.write_shard(shard_id).await); 
        self.current_shard_id = Some(shard_id);       
    }

    async fn get_chunk_column(&mut self, chunk_column_position: ChunkColumnPosition) -> Option<&mut ChunkColumn> {
        let shard_id = chunk_column_position.shard(self.world_map.get_shard_count());

        self.ensure_shard(shard_id).await;

        if let Some(shard) = &mut self.current_shard {
            // Here, we use a reference to `shard` instead of trying to move it
            shard.get_mut(&chunk_column_position)
        } else {
            unreachable!("ensure shard always sets to current_shard the requested shard")
        }
    }

    async fn set_light_level(&mut self, position: LightPosition, level: u8) {
        unimplemented!();
    }

    async fn get_light_level(&mut self, position: LightPosition) -> u8 {
        unimplemented!();
    }

    pub async fn set_block(&mut self, block_position: BlockPosition, block: BlockWithState) {
        let mut to_explore = BinaryHeap::new();
        let position = LightPosition::from(block_position.clone());
        to_explore.extend(position.get_neighbors(24));
        while let Some(postion) = to_explore.pop() {

            if let Some(column) = self.get_chunk_column(position.clone().into()).await {
                let block = Block::from(column.get_block(position.clone().into()));

                if block.is_transparent() {
                    let highest_block = column.get_highest_block_at(&block_position.in_chunk_column());
                    let is_inside = highest_block > postion.clone().y as u16 + 1;
                    let new_level = if is_inside { postion.clone().y as u8 - block.light_absorption() - 1 } else { MAX_LIGHT_LEVEL };
                    let new_position = LightPositionInChunkColumn::from(postion.clone());

                    to_explore.extend(postion.clone().get_neighbors(24));                
                }          
            } 
        }

        // Clear locked chunks

    }


    pub async fn init_chunk_column_light(world_map: &'static WorldMap, chunk_column_position: ChunkColumnPosition) {

        // Clear locked chubks
    }
}

impl ChunkColumn {
    /// Init independant light means it will compute the light for all the chunk without considering the neighbour chunks.
    pub(super) fn init_independant_light(&mut self) {
        let _ = self.light.sky_light.set_region(self.get_highest_block() as usize + 1, ChunkColumn::MAX_HEIGHT as usize, self.light.sky_light.level);

        for x in 0..16 {
            for z in 0..16 {
                for y in self.get_highest_block_at(&BlockPositionInChunkColumn {
                    bx: x,
                    y: 0i32,
                    bz: z
                })..(self.get_highest_block() as u16) {
                    let _ = self.light.sky_light.set_level(
                        LightPositionInChunkColumn {
                            bx: x,
                            y: y as usize,
                            bz: z
                        }, self.light.sky_light.level);
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

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, MAX_LIGHT_LEVEL).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }).unwrap(), MAX_LIGHT_LEVEL);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }, 0).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 0 }).unwrap(), 0);

        data.set(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 0, bz: 1 }).unwrap(), 1);

        data.set(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }, MAX_LIGHT_LEVEL).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 0, by: 1, bz: 1 }).unwrap(), MAX_LIGHT_LEVEL);

        data.set(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 1, by: 1, bz: 1 }).unwrap(), 1);

        data.set(BlockPositionInChunk { bx: 2, by: 0, bz: 0 }, 1).unwrap();
        assert_eq!(data.get(BlockPositionInChunk { bx: 2, by: 0, bz: 0 }).unwrap(), 1);

        // Manual layer
        for z in 0..16 {
            for x in 0..16 {
                data.set(BlockPositionInChunk { bx: x, by: 0, bz: z }, MAX_LIGHT_LEVEL).unwrap();
            }
        }

        for z in 0..16 {
            for x in 0..16 {
                assert_eq!(data.get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), MAX_LIGHT_LEVEL, "x: {}, z: {}", x, z);
            }
        }

        // Test layer
        data.set_layer(1, MAX_LIGHT_LEVEL).unwrap();
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(data.get(BlockPositionInChunk { bx: x, by: 1, bz: z }).unwrap(), MAX_LIGHT_LEVEL, "x: {}, z: {}", x, z);
            }
        }
    }

    #[test]
    fn test_set_region() {
        let mut sky_light = LightSystem {
            level: MAX_LIGHT_LEVEL,
            light_arrays: vec![SectionLightData::new(); 16+2],
            light_mask: 0,
            empty_light_mask: !0,
        };

        sky_light.set_region(1, 33, MAX_LIGHT_LEVEL).unwrap();

        // Test in
        assert_eq!(sky_light.light_arrays[0].get(BlockPositionInChunk { bx: 0, by: 1, bz: 7 }).unwrap(), MAX_LIGHT_LEVEL);
        assert_eq!(sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 1, by: MAX_LIGHT_LEVEL, bz: 8 }).unwrap(), MAX_LIGHT_LEVEL);
        assert_eq!(sky_light.light_arrays[2].get(BlockPositionInChunk { bx: 3, by: 0, bz: 0 }).unwrap(), MAX_LIGHT_LEVEL);

        // Test out
        assert_eq!(sky_light.light_arrays[0].get(BlockPositionInChunk { bx: 4, by: 0, bz: 2 }).unwrap(), 0);
        assert_eq!(sky_light.light_arrays[3].get(BlockPositionInChunk { bx: 0, by: 14, bz: 9 }).unwrap(), 0);
        assert_eq!(sky_light.light_arrays[4].get(BlockPositionInChunk { bx: 9, by: 0, bz: 10 }).unwrap(), 0);
    }

}
