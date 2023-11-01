use std::collections::HashMap;
use minecraft_protocol::components::chunk::Chunk;
use tokio::sync::RwLock;
use crate::prelude::*;

pub struct WorldMap {
    chunks: RwLock<HashMap<ChunkPosition, RwLock<ChunkColumn>>>,
}

struct ChunkColumn {
    chunks: Vec<Chunk>,
}

impl WorldMap {
    pub async fn load(&self, position: ChunkPosition) {
        todo!()
    }

    pub async fn unload(&self, position: ChunkPosition) {
        let mut chunks = self.chunks.write().await;
        chunks.remove(&position);
    }
}

