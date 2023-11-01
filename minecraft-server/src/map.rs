use std::collections::HashMap;
use minecraft_protocol::components::chunk::Chunk;
use tokio::sync::RwLock;

pub struct WorldMap {
    chunks: RwLock<HashMap<(i32, i32), ChunkColumn>>,
}

struct ChunkColumn {
    chunks: Vec<Chunk>,
}
