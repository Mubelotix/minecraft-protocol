use std::collections::HashMap;
use minecraft_protocol::{components::chunk::{Chunk as ChunkData, PalettedData}, ids::block_states::BlockWithState};
use tokio::sync::RwLock;
use crate::prelude::*;

pub struct WorldMap {
    /// The map is divided in shards.
    /// Chunks are evenly distributed between shards.
    /// The shards are locked independently.
    /// This allows high concurrency.
    shard_count: usize,
    shards: Vec<RwLock<HashMap<ChunkColumnPosition, ChunkColumn>>>,
}

impl ChunkColumnPosition {
    fn shard(&self, shard_count: usize) -> usize {
        (self.cx + self.cz).unsigned_abs() as usize % shard_count
    }
}

#[derive(Clone)]
struct Chunk {
    data: ChunkData,
    palette_block_counts: Vec<u16>,
}

impl Chunk {
    fn filled(block: BlockWithState) -> Option<Chunk> {
        Some(Chunk {
            data: ChunkData {
                block_count: 4096,
                blocks: PalettedData::Single { value: block.block_state_id()? },
                biomes: PalettedData::Single { value: 0 },
            },
            palette_block_counts: Vec::new(),
        })
    }

    fn from_chunk_data(data: ChunkData) -> Chunk {
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

    fn get_block(&self, position: BlockPositionInChunk) -> BlockWithState {
        match &self.data.blocks {
            PalettedData::Paletted { palette, indexed } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                let palette_position = indexed[data_position as usize];
                let block_state_id = palette[palette_position as usize];
                BlockWithState::from_state_id(block_state_id)
            },
            PalettedData::Single { value } => {
                BlockWithState::from_state_id(*value)
            }
            PalettedData::Raw { values } => {
                let data_position = position.by as usize * 16 * 16 + position.bz as usize * 16 + position.bx as usize;
                let block_state_id = values[data_position as usize];
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

struct ChunkColumn {
    chunks: Vec<Chunk>,
}

impl ChunkColumn {
    pub fn flat() -> Self {
        let empty_chunk = Chunk {
            data: ChunkData {
                block_count: 0,
                blocks: PalettedData::Single { value: 0 },
                biomes: PalettedData::Single { value: 4 },
            },
            palette_block_counts: Vec::new(),
        };
        let dirt_chunk = Chunk {
            data: ChunkData {
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
        ChunkColumn { chunks }
    }

    fn get_block(&self, position: BlockPositionInChunkColumn) -> BlockWithState {
        fn get_block_inner(s: &ChunkColumn, position: BlockPositionInChunkColumn) -> Option<BlockWithState> {
            let cy = position.cy();
            let cy_in_vec: usize = cy.saturating_add(4).try_into().ok()?;
            let position = position.in_chunk();
            let chunk = s.chunks.get(cy_in_vec)?;
            Some(chunk.get_block(position))
        }
        get_block_inner(self, position).unwrap_or(BlockWithState::Air)
    }

    fn set_block(&mut self, position: BlockPositionInChunkColumn, block: BlockWithState) {
        fn set_block_innter(s: &mut ChunkColumn, position: BlockPositionInChunkColumn, block: BlockWithState) -> Option<()> {
            let cy = position.cy();
            let cy_in_vec: usize = cy.saturating_add(4).try_into().ok()?;
            let position = position.in_chunk();
            let chunk = s.chunks.get_mut(cy_in_vec)?;
            chunk.set_block(position, block);
            Some(())
        }
        set_block_innter(self, position, block);
    }
}

impl WorldMap {
    pub fn new(shard_count: usize) -> WorldMap {
        let mut shards = Vec::new();
        for _ in 0..shard_count {
            shards.push(RwLock::new(HashMap::new()));
        }
        WorldMap { shard_count, shards }
    }

    pub async fn get_block(&self, position: BlockPosition) -> BlockWithState {
        async fn inner_get_block(s: &WorldMap, position: BlockPosition) -> Option<BlockWithState> {
            let chunk_position = position.chunk();
            let position_in_chunk_column = position.in_chunk_column();
            let chunk_column_position = chunk_position.chunk_column();
            let shard = chunk_column_position.shard(s.shard_count);
        
            let shard = s.shards[shard].read().await;
            let chunk_column = shard.get(&chunk_column_position)?;
            Some(chunk_column.get_block(position_in_chunk_column))
        }
        inner_get_block(self, position).await.unwrap_or(BlockWithState::Air)
    }

    pub async fn load(&self, position: ChunkColumnPosition) {
        let chunk = ChunkColumn::flat(); // TODO: load from disk
        let shard = position.shard(self.shard_count);
        
        let mut shard = self.shards[shard].write().await;
        shard.entry(position).or_insert_with(|| chunk);
    }

    pub async fn unload(&self, position: ChunkColumnPosition) {
        let shard = position.shard(self.shard_count);

        let mut shard = self.shards[shard].write().await;
        shard.remove(&position);
        // TODO: write to disk
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
}
