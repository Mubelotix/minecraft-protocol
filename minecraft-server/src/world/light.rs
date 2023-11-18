use std::{collections::BinaryHeap, ops::AddAssign};

use minecraft_protocol::ids::blocks::Block;

use crate::prelude::*;
use super::*;

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
pub struct EdgesLightToPropagate {
    pub edges: [BinaryHeap<(LightPositionInChunkColumn, u8)>; 4],
}

impl EdgesLightToPropagate {
    pub fn new() -> Self {
        Self {
            edges: [BinaryHeap::new(), BinaryHeap::new(), BinaryHeap::new(), BinaryHeap::new()],
        }
    }

    pub fn push(&mut self, position: LightPositionInChunkColumn, level: u8) {
        let index = match position {
            LightPositionInChunkColumn { bx: 0, y: _, bz: _ } => 0,
            LightPositionInChunkColumn { bx: _, y: _, bz: 0 } => 1,
            LightPositionInChunkColumn { bx: 15, y: _, bz: _ } => 2,
            LightPositionInChunkColumn { bx: _, y: _, bz: 15 } => 3,
            _ => return,
        };
        self.edges[index].push((position, level));
    }

    pub fn pop(&mut self) -> Option<(LightPositionInChunkColumn, u8)> {
        for edge in self.edges.iter_mut() {
            if let Some((position, level)) = edge.pop() {
                return Some((position, level));
            }
        }
        None
    }

    pub fn expand(&mut self, edges: EdgesLightToPropagate) {
        for (i, edge) in edges.edges.iter().enumerate() {
            self.edges[i].extend(edge.clone());
        }
    }

    pub fn chunk_positions_to_propagate(&self, from: ChunkColumnPosition) -> Vec<(ChunkColumnPosition, BinaryHeap<(LightPositionInChunkColumn, u8)>)> {
        let mut result = Vec::new();
        if !self.edges[0].is_empty() {
            result.push((from.clone() + ChunkColumnPosition { cx: -1, cz: 0 }, self.edges[0].clone()));
        }
        if !self.edges[1].is_empty() {
            result.push((from.clone() + ChunkColumnPosition { cx: 0, cz: -1 }, self.edges[1].clone()));
        }
        if !self.edges[2].is_empty() {
            result.push((from.clone() + ChunkColumnPosition { cx: 1, cz: 0 }, self.edges[2].clone()));
        }
        if !self.edges[3].is_empty() {
            result.push((from.clone() + ChunkColumnPosition { cx: 0, cz: 1 }, self.edges[3].clone()));
        }

        result
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
    edge_light_to_propagate: EdgesLightToPropagate,
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
    pub fn set_region(&mut self, from_y: usize, to_y: usize, level: u8) -> Result<EdgesLightToPropagate, ()> {
        if level > self.level {
            return Err(());
        }

        // Get the range of sections to set.
        let first_section = from_y.div_euclid(16);
        let first_secion_offset = from_y.rem_euclid(16);

        let last_section = to_y.div_euclid(16);
        let last_section_offset = to_y.rem_euclid(16);

        let mut edges = EdgesLightToPropagate::new();

        for section in first_section..=last_section {
            if section != first_section && section != last_section {
                // Set the whole section
                self.light_arrays[section].set_with(level);
                for y in 0..16 {
                    for i in 0..16 {
                        edges.push(LightPositionInChunkColumn { bx: i, y: section * 16 + y, bz: 0 }, level);
                        edges.push(LightPositionInChunkColumn { bx: i, y: section * 16 + y, bz: 15 }, level);
                        edges.push(LightPositionInChunkColumn { bx: 0, y: section * 16 + y, bz: i }, level);
                        edges.push(LightPositionInChunkColumn { bx: 15, y: section * 16 + y, bz: i }, level);
                    }
                }
            } else {
                // Set the part of the section
                let first_offset = if section == first_section { first_secion_offset } else { 0 };
                let last_offset = if section == last_section { last_section_offset } else { 15 };
                for y in first_offset..=last_offset {
                    self.light_arrays[section].set_layer(y as u8, level)?;
                    for i in 0..16 {
                        edges.push(LightPositionInChunkColumn { bx: i, y: section * 16 + y, bz: 0 }, level);
                        edges.push(LightPositionInChunkColumn { bx: i, y: section * 16 + y, bz: 15 }, level);
                        edges.push(LightPositionInChunkColumn { bx: 0, y: section * 16 + y, bz: i }, level);
                        edges.push(LightPositionInChunkColumn { bx: 15, y: section * 16 + y, bz: i }, level);
                    }
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

        Ok(edges)
    }

    pub(super) fn get_level(&self, position: LightPositionInChunkColumn) -> Result<u8, ()> {
        let section = position.y.div_euclid(16);
        self.light_arrays[section.max(0)].get(position.in_chunk())
    }

    pub(super) fn set_level(&mut self, position: LightPositionInChunkColumn, level: u8) -> Result<EdgesLightToPropagate, ()> {
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
        let mut edges = EdgesLightToPropagate::new();
        edges.push(position, level);
        Ok(edges)
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
                level: 15,
                light_arrays: vec![SectionLightData::new(); 24+2],
                light_mask: 0,
                empty_light_mask: !0,
                edge_light_to_propagate: EdgesLightToPropagate::new(),
            },
        }
    }

    pub fn get_packet(&self) -> (Array<Array<u8, VarInt>, VarInt>, BitSet, BitSet) {
        self.sky_light.get_packet_data()
    }

}

#[derive(Debug, Clone)]
pub struct LightPositionInChunkColumn {
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

    pub fn get_neighbors(&self, n_chunk: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();
        if self.y < ((n_chunk - 1) * 16) + 1 { // No block can be higher so no block can affect the light level 
            neighbors.push(LightPositionInChunkColumn { bx: self.bx, y: self.y + 1, bz: self.bz });
        }
        if self.bx > 0 {
            neighbors.push(LightPositionInChunkColumn { bx: self.bx - 1, y: self.y, bz: self.bz });
        }
        if self.bx < 15 {
            neighbors.push(LightPositionInChunkColumn { bx: self.bx + 1, y: self.y, bz: self.bz });
        }
        if self.bz > 0 {
            neighbors.push(LightPositionInChunkColumn { bx: self.bx, y: self.y, bz: self.bz - 1 });
        }
        if self.bz < 15 {
            neighbors.push(LightPositionInChunkColumn { bx: self.bx, y: self.y, bz: self.bz + 1 });
        }
        if self.y > 0 {
            neighbors.push(LightPositionInChunkColumn { bx: self.bx, y: self.y - 1, bz: self.bz });
        }
        neighbors
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
    pub(super) fn init_light(&mut self) -> Result<EdgesLightToPropagate, ()> {
        self.propagate_sky_light_inside()
    }

    fn propagate_sky_light_inside(&mut self) -> Result<EdgesLightToPropagate, ()> {
        let mut to_propagate = EdgesLightToPropagate::new();
        // Set all highest blocks to the highest block
        let highest_blocks = self.get_highest_block();

        let max_y = self.light.sky_light.light_arrays.len() * 16 - 1;
        let mut to_explore: BinaryHeap<LightPositionInChunkColumn> = BinaryHeap::new();
        to_propagate.expand(self.light.sky_light.set_region(highest_blocks as usize + 16, max_y, self.light.sky_light.level)?);
        
        // Add all highest blocks to the queue
        for x in 0..16 {
            for z in 0..16 {
                let position = LightPositionInChunkColumn {
                    bx: x,
                    y: self.get_highest_block_at(&BlockPositionInChunkColumn { bx: x, y: 0, bz: z }) as usize + 16 + 1,
                    bz: z,
                };
                to_explore.push(position);
            } 
        }

        to_propagate.expand(self.explore_sky_light_from_heap(&mut to_explore).map_err(|_| error!("Error while updating light"))?);
        Ok(to_propagate)
    }

    fn explore_sky_light_from_heap(&mut self, to_explore: &mut BinaryHeap<LightPositionInChunkColumn>) -> Result<EdgesLightToPropagate, ()> {
        // We get the neighbors and determine the light source from them
        // The neighbor with the highest light level is the light source
        // So we explore from it
        let mut edges = EdgesLightToPropagate::new();
        while let Some(position) = to_explore.pop() {
            let neighbors = position.get_neighbors(self.light.sky_light.light_arrays.len());
            let my_level = self.light.sky_light.get_level(position.clone())?;

            for neighbor in neighbors {
                let neighbor_level = self.light.sky_light.get_level(neighbor.clone())?;
            
                let block = Block::from(self.get_block(neighbor.clone().into())); 
                if block.is_transparent() 
                    && (neighbor_level < my_level.saturating_sub(1)) 
                {
                    let highest_block = self.get_highest_block_at(&neighbor.clone().into()) + 16;
                    let is_inside = highest_block > neighbor.y as u16 + 1;
                    to_explore.push(neighbor.clone());
                    let new_level = if is_inside { my_level - 1 } else { self.light.sky_light.level };
                    edges.expand(self.light.sky_light.set_level(neighbor, new_level)?);
                }
            }
        }
        Ok(edges)
    }

    fn clear_lightsystem_from(&mut self, position: LightPositionInChunkColumn) -> Result<EdgesLightToPropagate, ()> {
        let mut to_explore: BinaryHeap<LightPositionInChunkColumn> = BinaryHeap::new();
        let mut edges = EdgesLightToPropagate::new();
        // We get the neighbors and determine the light source from them
        // The neighbor with the highest light level is the light source
        // then we clear from the other neighbors
        // if are equal, we have nothing to do
        
        let my_level = self.light.sky_light.get_level(position.clone())?;
        self.light.sky_light.set_level(position.clone(), my_level.saturating_sub(1))?; 
        to_explore.push(position.clone());
    
        while let Some(position) = to_explore.pop() {
            let neighbors = position.get_neighbors(self.light.sky_light.light_arrays.len());
            let my_level = self.light.sky_light.get_level(position.clone())?;
            let my_is_inside = self.get_highest_block_at(&position.clone().into()) + 16 > position.y as u16 + 1;

            for neighbor in neighbors {
                let neighbor_level = self.light.sky_light.get_level(neighbor.clone()).unwrap();
            
                let block = Block::from(self.get_block(neighbor.clone().into())); 

                if block.is_transparent() 
                    && ((my_is_inside && neighbor_level <= my_level) 
                        || (!my_is_inside && neighbor_level < my_level))
                {
                    let highest_block = self.get_highest_block_at(&neighbor.clone().into()) + 16;
                    let is_inside = highest_block > neighbor.y as u16 + 1;
                    to_explore.push(neighbor.clone());
                    let new_level = if is_inside { my_level - block.light_absorption() - 1 } else { self.light.sky_light.level };
                    edges.expand(self.light.sky_light.set_level(neighbor, new_level)?);
                }
            }
        }
        Ok(edges)
    }

    pub(super) fn update_light_as_block_changed_at(&mut self, position: BlockPositionInChunkColumn) -> Result<EdgesLightToPropagate, ()> {
        let position = LightPositionInChunkColumn::from(position);
        let blocking = !Block::from(self.get_block(position.clone().into())).is_transparent();
        let mut to_explore: BinaryHeap<LightPositionInChunkColumn> = BinaryHeap::from(position.get_neighbors(self.light.sky_light.light_arrays.len()));
        let mut to_propagate = EdgesLightToPropagate::new();
        if blocking {
            to_propagate.expand(self.clear_lightsystem_from(position.clone()).map_err(|_| error!("Error while updating light"))?);
        } 
        to_propagate.expand(self.explore_sky_light_from_heap(&mut to_explore).map_err(|_| error!("Error while updating light"))?);
        Ok(to_propagate)
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
        let mut sky_light = LightSystem {
            level: 15,
            light_arrays: vec![SectionLightData::new(); 16+2],
            light_mask: 0,
            empty_light_mask: !0,
            edge_light_to_propagate: EdgesLightToPropagate::new(),
        };

        sky_light.set_region(1, 33, 15).unwrap();
        
        // Test in
        assert_eq!(sky_light.light_arrays[0].get(BlockPositionInChunk { bx: 0, by: 1, bz: 7 }).unwrap(), 15);
        assert_eq!(sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 1, by: 15, bz: 8 }).unwrap(), 15);
        assert_eq!(sky_light.light_arrays[2].get(BlockPositionInChunk { bx: 3, by: 0, bz: 0 }).unwrap(), 15);

        // Test out
        assert_eq!(sky_light.light_arrays[0].get(BlockPositionInChunk { bx: 4, by: 0, bz: 2 }).unwrap(), 0);
        assert_eq!(sky_light.light_arrays[3].get(BlockPositionInChunk { bx: 0, by: 14, bz: 9 }).unwrap(), 0);
        assert_eq!(sky_light.light_arrays[4].get(BlockPositionInChunk { bx: 9, by: 0, bz: 10 }).unwrap(), 0);
    }

    #[test]
    fn test_sky_light_flat_chunk() {
        let mut flat_chunk = ChunkColumn::flat();

        // Check that the sky light is equal to the light level above the grass and on the top of the world.
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(flat_chunk.light.sky_light.light_arrays[0].get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), 0);
                assert_eq!(flat_chunk.light.sky_light.light_arrays[4].get(BlockPositionInChunk { bx: x, by: 0, bz: z }).unwrap(), 15);
                assert_eq!(flat_chunk.light.sky_light.light_arrays[25].get(BlockPositionInChunk { bx: x, by: 15, bz: z }).unwrap(), 15);
            }
        }
        
        // Break the grass block and check that the sky light is correct.
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 15, bz: 0 }).unwrap(), 0);
        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -49, bz: 0 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 15, bz: 0 }).unwrap(), 15);
        
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 0 }).unwrap(), 0);
        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 0 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 0 }).unwrap(), 15);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 1 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 1 }).unwrap(), 14);
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 0, bz: 10 }).unwrap(), 0);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -50, bz: 2 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 14, bz: 2 }).unwrap(), 13);

        flat_chunk.set_block_for_test(BlockPositionInChunkColumn { bx: 0, y: -51, bz: 2 }, Block::Air.into());
        assert_eq!(flat_chunk.light.sky_light.light_arrays[1].get(BlockPositionInChunk { bx: 0, by: 13, bz: 2 }).unwrap(), 12);
    }
}
