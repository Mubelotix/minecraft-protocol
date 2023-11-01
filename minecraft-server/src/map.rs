use std::collections::HashMap;
use minecraft_protocol::components::chunk::{Chunk as ChunkData, PalettedData};
use tokio::sync::RwLock;
use crate::prelude::*;

pub struct WorldMap {
    chunks: RwLock<HashMap<ChunkPosition, RwLock<ChunkColumn>>>,
}

#[derive(Clone)]
struct Chunk {
    data: ChunkData,
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
    pub async fn load(&self, position: ChunkPosition) {
        let chunk = ChunkColumn::flat(); // TODO: load from disk
        let mut chunks = self.chunks.write().await;
        chunks.entry(position).or_insert_with(|| RwLock::new(chunk));
    }

    pub async fn unload(&self, position: ChunkPosition) {
        let mut chunks = self.chunks.write().await;
        chunks.remove(&position);
        // TODO: write to disk
    }
}
