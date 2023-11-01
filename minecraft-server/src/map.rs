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
        (self.cx + self.cz).abs() as usize % shard_count
    }
}

#[derive(Clone)]
struct Chunk {
    data: ChunkData,
}

impl Chunk {
    fn get_block(&self, position: BlockPositionInChunk) -> Option<BlockWithState> {
        match &self.data.blocks {
            PalettedData::Paletted { palette, indexed } => {
                let block_index = position.by * 16 * 16 + position.bz * 16 + position.bx;
                let block_palette_index = indexed[block_index as usize];
                let block_state_id = palette[block_palette_index as usize];
                BlockWithState::from_state_id(block_state_id)
            },
            PalettedData::Single { value } => {
                BlockWithState::from_state_id(*value)
            }
            PalettedData::Raw { values } => {
                let block_index = position.by * 16 * 16 + position.bz * 16 + position.bx;
                let block_state_id = values[block_index as usize];
                BlockWithState::from_state_id(block_state_id)
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
            }
        };
        let dirt_chunk = Chunk {
            data: ChunkData {
                block_count: 4096,
                blocks: PalettedData::Single { value: minecraft_protocol::ids::blocks::Block::GrassBlock.default_state_id() },
                biomes: PalettedData::Single { value: 4 },
            }
        };
        let mut chunks = Vec::new();
        chunks.push(dirt_chunk);
        for _ in 0..23 {
            chunks.push(empty_chunk.clone());
        }
        ChunkColumn { chunks }
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

    pub async fn get_block(&self, position: BlockPosition) -> Option<BlockWithState> {
        let chunk_position = position.chunk();
        let block_position_in_chunk = position.in_chunk();
        let chunk_column_position = chunk_position.chunk_column();
        let shard = chunk_column_position.shard(self.shard_count);

        let shard = self.shards[shard].read().await;
        let chunk_column = shard.get(&chunk_column_position)?;
        let chunk = chunk_column.chunks.get(chunk_position.cy as usize)?;
        chunk.get_block(block_position_in_chunk)
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
