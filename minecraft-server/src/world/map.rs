use std::{collections::HashMap, cmp::Ordering, vec};
use minecraft_protocol::{components::chunk::PalettedData, ids::blocks::Block};
use tokio::sync::{RwLock, OwnedRwLockWriteGuard};
use crate::{prelude::*, world::light::LightManager};
use super::light::Light;


pub struct WorldMap {
    /// The map is divided in shards.
    /// Chunks are evenly distributed between shards.
    /// The shards are locked independently.
    /// This allows high concurrency.
    shard_count: usize,
    shards: Vec<Arc<RwLock<HashMap<ChunkColumnPosition, ChunkColumn>>>>,
}

#[derive(Clone)]
pub(super) struct Chunk {
    data: NetworkChunk,
    palette_block_counts: Vec<u16>,
}

impl Chunk {
    fn filled(block: BlockWithState) -> Option<Chunk> {
        Some(Chunk {
            data: NetworkChunk {
                block_count: 4096,
                blocks: PalettedData::Single { value: block.block_state_id()? },
                biomes: PalettedData::Single { value: 0 },
            },
            palette_block_counts: Vec::new(),
        })
    }

    fn from_chunk_data(data: NetworkChunk) -> Chunk {
        let mut palette_block_counts = Vec::new();
        if let PalettedData::Paletted { palette, indexed } = &data.blocks {
            palette_block_counts = vec![0; palette.len()];
            for index in indexed.iter().copied().map(|i| i as usize) {
                if index < palette.len() {
                    palette_block_counts[index] += 1;
                } else {
                    // TODO
                }
            }
        }
        Chunk {
            data,
            palette_block_counts
        }
    }

    fn as_network_chunk(&self) -> &NetworkChunk {
        &self.data
    }

    fn get_block(&self, position: BlockPositionInChunk) -> BlockWithState {
        match &self.data.blocks {
            PalettedData::Paletted { palette, indexed } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                let palette_position = indexed[data_position];
                let block_state_id = palette[palette_position as usize];
                BlockWithState::from_state_id(block_state_id)
            },
            PalettedData::Single { value } => {
                BlockWithState::from_state_id(*value)
            }
            PalettedData::Raw { values } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                let block_state_id = values[data_position];
                BlockWithState::from_state_id(block_state_id)
            }
        }.unwrap_or(BlockWithState::Air)
    }

    // TODO edit block_count in data
    fn set_block(&mut self, position: BlockPositionInChunk, block: BlockWithState) {
        let block_state_id = block.block_state_id().unwrap_or_else(|| {
            error!("Tried to set block with invalid state {block:?}. Placing air"); 0
        });
        match &mut self.data.blocks {
            PalettedData::Paletted { palette, indexed } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;

                // Decrease count of previous block
                let prev_palette_index = indexed[data_position] as usize;
                if let Some(prev_count) = self.palette_block_counts.get_mut(prev_palette_index) {
                    *prev_count -= 1;
                }

                // Truncate palette by removing all unused blocks from the right
                while self.palette_block_counts.last().map(|c| *c==0).unwrap_or(false) {
                    self.palette_block_counts.truncate(self.palette_block_counts.len() - 1);
                    palette.truncate(self.palette_block_counts.len());
                }

                // Find position in palette for new block
                let position = 'find_pos: {
                    // Find existing position in palette
                    let position = palette.iter().position(|in_palette| *in_palette == block_state_id);
                    if position.is_some() {
                        break 'find_pos position;
                    }

                    // Find replaceable position in palette
                    let position = self.palette_block_counts.iter().position(|count| *count==0);
                    if let Some(position) = position {
                        palette[position] = block_state_id;
                        break 'find_pos Some(position);
                    }

                    // Add to palette if there is still place
                    let position = palette.len();
                    if position <= 0xff {
                        palette.push(block_state_id);
                        self.palette_block_counts.push(0);
                        break 'find_pos Some(position);
                    }

                    None
                };

                // If palette lenght is one, turn to single-valued
                if palette.len() == 1 {
                    self.data.blocks = PalettedData::Single { value: block_state_id };
                    self.palette_block_counts.clear();
                    return;
                }

                match position {
                    Some(palette_position) => {
                        // Add block and increase its count
                        indexed[data_position] = palette_position as u8;
                        if let Some(count) = self.palette_block_counts.get_mut(palette_position) {
                            *count += 1;
                        }
                    },
                    None => {
                        // Turn to raw
                        let mut values = Vec::new();
                        for palette_index in indexed.iter().copied() {
                            let value = palette.get(palette_index as usize).copied().unwrap_or_default();
                            values.push(value);
                        }
                        values[data_position] = block_state_id;
                        self.data.blocks = PalettedData::Raw { values };
                        self.palette_block_counts.clear();
                    }
                }
            },
            PalettedData::Single { ref value } => {
                if block_state_id == *value {
                    return;
                }

                // Turn to paletted values
                let palette = vec![*value, block_state_id];
                let mut indexed = vec![0; 4096];
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                indexed[data_position] = 1;
                self.data.blocks = PalettedData::Paletted { palette, indexed };
                self.palette_block_counts = vec![4095, 1];
            }
            PalettedData::Raw { values } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                values[data_position] = block_state_id;
            }
        }
    }
}

struct HeightMap {
    base: u8,
    data: Vec<u64>,
    max_height: Option<u32>,
}

impl HeightMap {
    pub fn new(base: u8) -> Self {
        assert!(base <= 9, "base must be <= 9 because the max height is 320 + 64"); 
        Self {
            base,
            data: vec![0; ((16 * 16 * 9usize).div_euclid(base as usize) + 1) * base as usize ],
            max_height: None
        }
    }
    
    pub fn to_tag(&self) -> NbtTag {
        NbtTag::Compound(
            HashMap::from_iter(
                vec![
                    (String::from("MOTION_BLOCKING"), NbtTag::LongArray(unsafe {
                        std::mem::transmute::<Vec<u64>, Vec<i64>>(self.data.clone())
                    })),
                ]
            )
        )
    }

    /// Update the current base of the heightmap.
    fn new_base(&mut self, new_base: u8) {
        assert!(new_base <= 9, "base must be <= 9 because the max height is 320 + 64");

        let old_base = self.base as usize;

        unimplemented!();

        self.base = new_base as u8;
    }


    fn get_need_base(&self, height: u32) -> u8 {
        32 - ((height + 1).leading_zeros() as u8)
    }

    /// Set the height of the highest block at the given position.
    pub fn set(&mut self, position: &BlockPositionInChunkColumn, height: u32) {
        let (x, z) = (position.bx, position.bz);
        // Check if the height is higher than the current max height.
        if let Some(max_height) = self.max_height {
            if height < max_height {        // Calculate the new base for the data.
                let new_base = self.get_need_base(height);
                // Update the base & max height.
                self.max_height = Some(height);
            }
        } else {
            // Set the max height.
            self.max_height = Some(height);
        }
        
        let index = (x * 16 + z) as usize; // assuming a 16x16 chunk column
        let bits_per_entry = self.base as usize;
        let bit_pos = index * bits_per_entry;
        let data_index = bit_pos / 64;
        let bit_offset = bit_pos % 64;

        // Ensure we don't shift beyond the limits of the data type.
        if bits_per_entry >= 64 {
            panic!("base too large for u64 storage");
        }

        // Cast the height to u64 
        let height = height as u64;

        // Prepare the mask to clear the bits at the position.
        let mask = ((1 << bits_per_entry) - 1) << bit_offset;
        // Clear the bits at the target position.
        self.data[data_index] &= !mask;
        // Set the new height with the sign.
        self.data[data_index] |= height << bit_offset;
        // Check if the entry spills over to the next u64.
        if bit_offset + bits_per_entry > 64 {
            // Calculate how many bits spill over.
            let spill_over_bits = bit_offset + bits_per_entry - 64;
            // Prepare the mask to clear the spill over bits.
            let spill_over_mask = (1 << spill_over_bits) - 1;
            // Clear the spill over bits in the next entry.
            self.data[data_index + 1] &= !spill_over_mask;
            // Set the spill over bits.
            self.data[data_index + 1] |= height >> (64 - bit_offset);
        }
    }
    
    /// Get the height of the highest block at the given position.
    pub fn get(&self, position: &BlockPositionInChunkColumn) -> u16 {
        let (x, z) = (position.bx, position.bz);

        let index = (x * 16 + z) as usize; // assuming a 16x16 chunk column
        let bits_per_entry = self.base as usize;
        let bit_pos = index * bits_per_entry;
        let data_index = bit_pos / 64;
        let bit_offset = bit_pos % 64;

        // Prepare the mask to get the bits at the position.
        let mask = ((1u64 << bits_per_entry) - 1) << bit_offset;
        // Retrieve the bits.
        let mut value = (self.data[data_index] & mask) >> bit_offset;

        // Check if the entry spills over to the next u64 and retrieve the remaining bits.
        if bit_offset + bits_per_entry > 64 {
            // Calculate how many bits spill over.
            let spill_over_bits = bit_offset + bits_per_entry - 64;
            // Prepare the mask to get the spill over bits.
            let spill_over_mask = (1u64 << spill_over_bits) - 1;
            // Retrieve the spill over bits from the next entry.
            value |= (self.data[data_index + 1] & spill_over_mask) << (64 - bit_offset);
        }

        // Perform sign extension if the value is negative.
        let sign_bit = 1u64 << (bits_per_entry - 1);
        if value & sign_bit != 0 {
            // If the sign bit is set, extend the sign to the rest of the i64.
            value |= !((1u64 << bits_per_entry) - 1);
        }

        // Cast to i32 with sign extension.
        value as u16
    }
 
}


pub(super) struct ChunkColumn {
    heightmap: HeightMap,
    pub(super) light: Light,
    chunks: Vec<Chunk>,
}

impl ChunkColumn {
    pub const MAX_HEIGHT: u16 = 320 + 64; // TODO: adapt to the world height
    pub const MIN_Y: i32 = -64;

    fn init_chunk_heightmap(&mut self){
        self.heightmap = HeightMap::new(9);
        if self.chunks.len() != 24 {
            panic!("Chunk column must have 24 chunks (change it for other world heights)");
        }

        // Start from the higher chunk
        for bx in 0..16 {
            for bz in 0..16 {
                let height = self.get_higher_skylight_filter_block(&BlockPositionInChunkColumn { bx, y: 0, bz }, Self::MAX_HEIGHT).into();
                self.heightmap.set(&BlockPositionInChunkColumn { bx, y: 0, bz }, height);
            }
        }
    }

    fn get_higher_skylight_filter_block(&self, position: &BlockPositionInChunkColumn, current_height: u16) -> u16 {
        let n_chunk_to_skip = self.chunks.len() - current_height.div_euclid(16) as usize - (current_height.rem_euclid(16) > 0) as usize;
        let mut current_height = current_height - 1;
        // Downward propagation
        for chunk in self.chunks.iter().rev().skip(n_chunk_to_skip) {
            for by in (0..((((current_height) % 16) + 1) as u8)).rev() {
                let block: BlockWithState = chunk.get_block(BlockPositionInChunk { bx: position.bx, by, bz: position.bz });
                // SAFETY: fom_id will get a valid block necessarily 
                if !Block::from(block).is_transparent() {
                    return current_height + 1;
                }
                current_height = current_height.saturating_sub(1);
            }          
        }
        current_height
    }
    
    pub(super) fn get_highest_block(&self) -> u32 {
        self.heightmap.max_height.unwrap_or(0)
    }

    pub(super) fn get_highest_block_at(&self, position: &BlockPositionInChunkColumn) -> u16 {
        self.heightmap.get(position) 
    }

    pub fn from(chunks: Vec<Chunk>) -> Self {
        let mut column = Self { 
            chunks, 
            heightmap: HeightMap::new(9),
            light: Light::new(),
        };
        column.init_chunk_heightmap();
        column.init_independant_light();
        column
    }

    pub fn flat() -> Self {
        let empty_chunk = Chunk {
            data: NetworkChunk {
                block_count: 0,
                blocks: PalettedData::Single { value: 0 },
                biomes: PalettedData::Single { value: 4 },
            },
            palette_block_counts: Vec::new(),
        };
        let dirt_chunk = Chunk {
            data: NetworkChunk {
                block_count: 4096,
                blocks: PalettedData::Single { value: minecraft_protocol::ids::blocks::Block::GrassBlock.default_state_id() },
                biomes: PalettedData::Single { value: 4 },
            },
            palette_block_counts: Vec::new(),
        };
        let mut chunks = Vec::new();
        chunks.push(dirt_chunk);
        for _ in 0..23 {
            chunks.push(empty_chunk.clone());
        }
        Self::from(chunks)
    }

    pub(super) fn get_block(&self, position: BlockPositionInChunkColumn) -> BlockWithState {
        fn get_block_inner(s: &ChunkColumn, position: BlockPositionInChunkColumn) -> Option<BlockWithState> {
            let cy = position.cy();
            let cy_in_vec: usize = cy.saturating_add(4).try_into().ok()?;
            let position = position.in_chunk();
            let chunk = s.chunks.get(cy_in_vec)?;
            Some(chunk.get_block(position))
        }
        get_block_inner(self, position).unwrap_or(BlockWithState::Air)
    }

    #[cfg(test)]
    pub fn set_block_for_test(&mut self, position: BlockPositionInChunkColumn, block: BlockWithState) {
        self.set_block(position, block);
    }

    fn set_block(&mut self, position: BlockPositionInChunkColumn, block: BlockWithState) {
        fn set_block_innter(s: &mut ChunkColumn, position: BlockPositionInChunkColumn, block: BlockWithState) -> Option<()> {
            let cy = position.cy();
            let cy_in_vec: usize = cy.saturating_add(4).try_into().ok()?;
            let position = position.in_chunk();
            let chunk = s.chunks.get_mut(cy_in_vec)?;
            chunk.set_block(position, block.clone());
            Some(())
        }
        set_block_innter(self, position.clone(), block.clone());

        let last_height = self.heightmap.get(&position);
        let not_filter_sunlight = Block::from(block.clone()).is_transparent(); // TODO: check if the block is transparent

        // Get the height of the placed block
        let block_height = (position.y - Self::MIN_Y + 1).max(0) as u16;
        match block_height.cmp(&last_height) {
            Ordering::Greater if !not_filter_sunlight => {
                self.heightmap.set(&position, block_height.into());
            },
            Ordering::Equal if not_filter_sunlight => {
                // Downward propagation
                let new_height = self.get_higher_skylight_filter_block(&position, last_height).into();
                self.heightmap.set(&position, new_height);
            },
            _ => {}   
        }
    }

    fn get_skylight(&self, position: BlockPositionInChunkColumn) -> u8 {
        self.light.get_skylight_level(position.into())
    }
}

impl WorldMap {
    pub fn new(shard_count: usize) -> WorldMap {
        let mut shards = Vec::new();
        for _ in 0..shard_count {
            shards.push(Arc::new(RwLock::new(HashMap::new())));
        }
        WorldMap { 
            shard_count,
            shards,
        }
    }

    pub async fn get_block(&self, position: BlockPosition) -> BlockWithState {
        async fn inner_get_block(s: &WorldMap, position: BlockPosition) -> Option<BlockWithState> {
            let chunk_position = position.chunk();
            let position_in_chunk_column: BlockPositionInChunkColumn = position.in_chunk_column();
            let chunk_column_position = chunk_position.chunk_column();
            let shard = chunk_column_position.shard(s.shard_count);
        
            let shard = s.shards[shard].read().await;
            let chunk_column = shard.get(&chunk_column_position)?;
            Some(chunk_column.get_block(position_in_chunk_column))
        }
        inner_get_block(self, position).await.unwrap_or(BlockWithState::Air)
    }

    pub(super) async fn write_shard(&self, shard: usize) -> OwnedRwLockWriteGuard<HashMap<ChunkColumnPosition, ChunkColumn>> {
        self.shards[shard].clone().write_owned().await
    }

    pub async fn get_network_chunk_column_data(&self, position: ChunkColumnPosition) -> Option<Vec<u8>> {
        let shard = position.shard(self.shard_count);
        let shard = self.shards[shard].read().await;
        let chunk_column = shard.get(&position)?;

        let serialized = NetworkChunk::into_data(chunk_column.chunks.iter().map(|c| c.data.clone()).collect()).unwrap();
        let (skylight_array_data, skylight_mask, empty_skylight_mask) = chunk_column.light.get_packet();

        let chunk_data = PlayClientbound::ChunkData { value: NetworkChunkColumnData {
            chunk_x: position.cx,
            chunk_z: position.cz,
            heightmaps: chunk_column.heightmap.to_tag(),
            data: Array::from(serialized.clone()),
            block_entities: Array::default(),
            sky_light_mask: skylight_mask,
            block_light_mask: Array::default(),
            empty_sky_light_mask: empty_skylight_mask,
            empty_block_light_mask: Array::default(),
            sky_light: skylight_array_data,
            block_light: Array::default(),
        }};
        let serialized = chunk_data.serialize_minecraft_packet().ok()?;
        Some(serialized)
    }
    
    pub async fn set_block(&'static self, position: BlockPosition, block: BlockWithState) {
        async fn inner_set_block(s: &'static WorldMap, position: BlockPosition, block: BlockWithState) -> Option<()> {
            let chunk_position = position.chunk();
            let position_in_chunk_column = position.in_chunk_column();
            let chunk_column_position = chunk_position.chunk_column();
            let shard = chunk_column_position.shard(s.shard_count);
        
            let mut shard = s.shards[shard].write().await;
            let chunk_column = shard.get_mut(&chunk_column_position)?;
            chunk_column.set_block(position_in_chunk_column.clone(), block);
            Some(())
        }

        inner_set_block(self, position.clone(), block.clone()).await;
        LightManager::update_light(self, position, block).await;
    }

    pub async fn get_skylight(&self, position: BlockPosition) -> u8 {
        async fn inner_get_skylight(s: &WorldMap, position: BlockPosition) -> Option<u8> {
            let chunk_position = position.chunk();
            let chunk_column_position = chunk_position.chunk_column();
            let shard = chunk_column_position.shard(s.shard_count);
        
            let shard = s.shards[shard].read().await;
            let chunk_column = shard.get(&chunk_column_position)?;
            let level = chunk_column.get_skylight(position.in_chunk_column());
            Some(level)
        }
        inner_get_skylight(self, position).await.unwrap_or(0)
    }

    /*async fn update_light_from_edge(&self, chunk_column_position: ChunkColumnPosition, to_propagate: BinaryHeap<(LightPositionInChunkColumn, u8)>) {
        async fn inner_get_skylight(s: &WorldMap, chunk_column_position: ChunkColumnPosition, to_propagate: BinaryHeap<(LightPositionInChunkColumn, u8)>) -> Option<()> {
            let shard = chunk_column_position.shard(s.shard_count);
        
            let mut shard = s.shards[shard].write().await;
            let chunk_column = shard.get_mut(&chunk_column_position)?;
            chunk_column.update_from_edge(to_propagate).ok()?;
            Some(())
        }        
        inner_get_skylight(self, chunk_column_position, to_propagate).await;
    }*/

    pub async fn try_move(&self, object: &CollisionShape, movement: &Translation) -> Translation {
        // TODO(perf): Optimize Map.try_move by preventing block double-checking
        // Also lock the map only once
        let movement_fragments = movement.clone().fragment(object);
        let mut validated = Translation{ x: 0.0, y: 0.0, z: 0.0 };
        for fragment in movement_fragments {
            let validating = validated.clone() + fragment;
            let translated_object = object.clone() + &validating;
            for block in translated_object.containing_blocks() {
                let block = self.get_block(block).await;
                if block.block_id() != 0 {
                    return validated;
                }
            }
            validated = validating;
        }
        movement.clone() // Would be more logic if it returned validated, but this way we avoid precision errors
    }

    pub async fn load(&'static self, position: ChunkColumnPosition) {
        let chunk = ChunkColumn::flat(); // TODO: load from disk
        let shard = position.shard(self.shard_count);
        
        trace!("Loading chunk column at {:?}", position);
        let mut shard = self.shards[shard].write().await;
        shard.entry(position.clone()).or_insert_with(|| chunk);
        LightManager::init_chunk_column_light(self, position).await;
    }

    pub async fn unload(&self, _position: ChunkColumnPosition) {
        // Note: these are not unloaded yet in order to preserve map data

        //let shard = position.shard(self.shard_count);

        //let mut shard = self.shards[shard].write().await;
        //shard.remove(&position);
        // TODO: write to disk
    }

    pub fn get_shard_count(&self) -> usize {
        self.shard_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_block() {
        let chunk = Chunk::filled(BlockWithState::Dirt).unwrap();
        chunk.get_block(BlockPositionInChunk { bx: 0, by: 1, bz: 2 });
    }

    #[test]
    fn test_set_block_paletted() {
        let mut chunk = Chunk::filled(BlockWithState::Dirt).unwrap();

        // Set enough blocks so that the chunk turns into paletted but not raw
        let mut id = 1;
        for bx in 0..16 {
            chunk.set_block(BlockPositionInChunk { bx, by: 0, bz: 0 }, BlockWithState::from_state_id(id).unwrap());
            id += 1;
        }
        assert!(!chunk.palette_block_counts.is_empty());
        let mut id = 1;
        for bx in 0..16 {
            let got = chunk.get_block(BlockPositionInChunk { bx, by: 0, bz: 0 }).block_state_id().unwrap();
            assert_eq!(id, got);
            id += 1;
        }
    }

    #[test]
    fn test_set_block_raw() {
        let mut chunk = Chunk::filled(BlockWithState::Dirt).unwrap();

        // Set enough blocks so that it turns to raw values
        let mut id = 1;
        for bx in 0..16 {
            for by in 0..16 {
                for bz in 0..2 {
                    chunk.set_block(BlockPositionInChunk { bx, by, bz }, BlockWithState::from_state_id(id).unwrap());
                    id += 1;
                }
            }
        }
        assert!(chunk.palette_block_counts.is_empty());
        let mut id = 1;
        for bx in 0..16 {
            for by in 0..16 {
                for bz in 0..2 {
                    let got = chunk.get_block(BlockPositionInChunk { bx, by, bz }).block_state_id().unwrap();
                    assert_eq!(id, got);
                    id += 1;
                }
            }
        }
    }

    #[test]
    fn test_palette_shrinking() {
        let mut chunk = Chunk::filled(BlockWithState::Air).unwrap();

        // Increase palette size by 16
        let mut id = 1;
        for bx in 0..16 {
            chunk.set_block(BlockPositionInChunk { bx, by: 0, bz: 0 }, BlockWithState::from_state_id(id).unwrap());
            id += 1;
        }
        assert_eq!(chunk.palette_block_counts.len(), 17);

        // Remove last block, it should decrease palete size
        chunk.set_block(BlockPositionInChunk { bx: 15, by: 0, bz: 0 }, BlockWithState::Air);
        assert_eq!(chunk.palette_block_counts.len(), 16);

        // Remove blocks at the start, palette cannot be shrinked from the left
        for bx in 0..8 {
            chunk.set_block(BlockPositionInChunk { bx, by: 0, bz: 0 }, BlockWithState::Air);
        }
        assert_eq!(chunk.palette_block_counts.len(), 16);

        // Remove remaining blocks but 1, palette will shrink and turn into single-valued
        for bx in 8..16 {
            chunk.set_block(BlockPositionInChunk { bx, by: 0, bz: 0 }, BlockWithState::Air);
        }
        assert_eq!(chunk.palette_block_counts.len(), 0);
    }

    #[test]
    fn test_chunk_column() {
        let mut flat_column = ChunkColumn::flat();

        let low_block = flat_column.get_block(BlockPositionInChunkColumn { bx: 0, y: -55, bz: 0 });
        assert_eq!(low_block.block_state_id().unwrap(), BlockWithState::GrassBlock { snowy: false }.block_state_id().unwrap());

        flat_column.set_block(BlockPositionInChunkColumn { bx: 0, y: -55, bz: 0 }, BlockWithState::Air);
        let low_block = flat_column.get_block(BlockPositionInChunkColumn { bx: 0, y: -55, bz: 0 });
        assert_eq!(low_block.block_state_id().unwrap(), BlockWithState::Air.block_state_id().unwrap());

        let too_low_block = flat_column.get_block(BlockPositionInChunkColumn { bx: 0, y: -65, bz: 0 });
        assert_eq!(too_low_block.block_state_id().unwrap(), BlockWithState::Air.block_state_id().unwrap());

        let high_block = flat_column.get_block(BlockPositionInChunkColumn { bx: 0, y: 120, bz: 0 });
        assert_eq!(high_block.block_state_id().unwrap(), BlockWithState::Air.block_state_id().unwrap());
    }

    #[tokio::test]
    async fn test_world_map() {
        let world_map = Box::leak(Box::new(WorldMap::new(1)));
        for cx in -3..=3 {
            for cz in -3..=3 {
                world_map.load(ChunkColumnPosition { cx, cz }).await;
            }
        }
        
        // Test single block
        world_map.set_block(BlockPosition { x: -40, y: -40, z: -40 }, BlockWithState::RedstoneBlock).await;
        let block = world_map.get_block(BlockPosition { x: -40, y: -40, z: -40 }).await;
        assert_eq!(block.block_state_id().unwrap(), BlockWithState::RedstoneBlock.block_state_id().unwrap());

        // Set blocks
        let mut id = 1;
        for x in (-40..40).step_by(9) {
            for y in (-40..200).step_by(15) {
                for z in (-40..40).step_by(9) {
                    world_map.set_block(BlockPosition { x, y, z }, BlockWithState::from_state_id(id).unwrap()).await;
                    id += 1;
                }
            }
        }

        // Verify they are set
        let mut id = 1;
        for x in (-40..40).step_by(9) {
            for y in (-40..200).step_by(15) {
                for z in (-40..40).step_by(9) {
                    let got = world_map.get_block(BlockPosition { x, y, z }).await.block_state_id().unwrap();
                    assert_eq!(id, got);
                    id += 1;
                }
            }
        }
    }

    #[test]
    fn test_heightmap_get_and_set() {
        let mut heightmap = HeightMap::new(5);
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }, 0);
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: -2, bz: 1 }, 2);
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: 3, bz: 2 }, 3);
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: -4, bz: 3 }, 4);
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: -4, bz: 7 }, 5);

        // Test get
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 0);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 1 }), 2);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 2 }), 3);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 3 }), 4);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 7 }), 5);

        // Test erase
        heightmap.set(&BlockPositionInChunkColumn { bx: 0, y: 12, bz: 0 }, 12);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 12, bz: 0 }), 12);

        // Test new base
        //heightmap.new_base(8);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 12);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 1 }), 2);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 2 }), 3);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 3 }), 4);
        assert_eq!(heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 7 }), 5);
    }

    #[test]
    fn test_heightmap_auto_updates() {
        let mut flat_column = ChunkColumn::flat();

        // Check that the heightmap is correct
        flat_column.set_block(BlockPositionInChunkColumn { bx: 0, y: 2, bz: 0 }, BlockWithState::GrassBlock { snowy: true });
        flat_column.init_chunk_heightmap();
        assert_eq!(flat_column.heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 67);
        assert_eq!(flat_column.heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 1 }), 16);

        // Now check that the heightmap is correct after setting a block
        flat_column.set_block(BlockPositionInChunkColumn { bx: 0, y: 10, bz: 0 }, BlockWithState::GrassBlock { snowy: false });
        assert_eq!(flat_column.heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 75);

        // Check that the heightmap is correct after setting a block to air under the highest block
        flat_column.set_block(BlockPositionInChunkColumn { bx: 0, y: 8, bz: 0 }, BlockWithState::Air);
        assert_eq!(flat_column.heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 75);

        // Check that the heightmap is correct after setting the highest block to air
        flat_column.set_block(BlockPositionInChunkColumn { bx: 0, y: 10, bz: 0 }, BlockWithState::Air);
        assert_eq!(flat_column.heightmap.get(&BlockPositionInChunkColumn { bx: 0, y: 0, bz: 0 }), 67);
    }

    #[test]
    fn benchmark_get_block() {
        let start_time = std::time::Instant::now();
        for _ in 0..441 {
            let _column = ChunkColumn::flat();
        }
      
        let elapsed: Duration = start_time.elapsed();
        println!("All Elapsed: {:?}", elapsed);
        println!("Elapsed: {:?}", elapsed / 441);
    }

    #[tokio::test]
    async fn test_try_move() {
        let world_map: &mut WorldMap = Box::leak(Box::new(WorldMap::new(1)));
        world_map.load(ChunkColumnPosition { cx: 0, cz: 0 }).await;
        let bounding_box = CollisionShape {
            x1: 0.0,
            y1: 0.0,
            z1: 0.0,
            x2: 1.0,
            y2: 1.0,
            z2: 1.0,
        };

        // Position on ground and try to go through it
        let positionned_box = bounding_box.clone() + &Translation { x: 0.0, y: -3.0*16.0, z: 0.0 };
        let movement = Translation { x: 0.0, y: -10.0, z: 0.0 };
        let movement = world_map.try_move(&positionned_box, &movement).await;
        assert_eq!(movement, Translation { x: 0.0, y: 0.0, z: 0.0 }); // It doesn't get through

        // Place it a little above ground
        let positionned_box = bounding_box.clone() + &Translation { x: 0.0, y: -3.0*16.0 + 1.0, z: 0.0 };
        let movement = Translation { x: 0.0, y: -10.0, z: 0.0 };
        let movement = world_map.try_move(&positionned_box, &movement).await;
        assert_eq!(movement, Translation { x: 0.0, y: -1.0, z: 0.0 }); // It falls down but doesn't get through

        // Place it above but not on round coordinates
        let positionned_box = bounding_box.clone() + &Translation { x: 0.0, y: -3.0*16.0 + 1.1, z: 0.2 };
        let movement = Translation { x: 2.0, y: -10.0, z: 0.0 };
        let movement = world_map.try_move(&positionned_box, &movement).await;
        assert_eq!(movement, Translation { x: 0.2200000000000003, y: -1.1000000000000014, z: 0.0 }); // It falls down but doesn't get through
    }

    #[tokio::test]
    async fn test_skylight() {
        let world_map = Box::leak(Box::new(WorldMap::new(1)));
        world_map.load(ChunkColumnPosition { cx: 0, cz: 0 }).await;

        // Test skylight initialisation for flat map 
        assert_eq!(world_map.get_skylight(BlockPosition { x: 8, y: 200, z: 8 }).await, 15, "The skylight is not valid for the blocks higher than the highest block");
    
    }
}
