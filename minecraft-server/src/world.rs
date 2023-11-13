
use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum WorldChange {
    BlockChange(BlockPosition, BlockWithState),
}

#[derive(Default)]
struct WorldLoadingManager {
    loaded_chunks: HashMap<UUID, HashSet<ChunkColumnPosition>>,
    loader_entities: HashMap<ChunkColumnPosition, HashSet<UUID>>,
}

impl WorldLoadingManager {
    fn update_loaded_chunks(&mut self, uuid: UUID, loaded_chunks: HashSet<ChunkColumnPosition>) {
        let loaded_before = self.loaded_chunks.entry(uuid).or_default();
        for just_unloaded in loaded_before.difference(&loaded_chunks) {
            let mut can_be_removed = false;
            self.loader_entities.entry(just_unloaded.clone()).and_modify(|f| {
                f.remove(&uuid);
                if f.is_empty() { can_be_removed = true;}
            });
            if can_be_removed {
                self.loader_entities.remove(just_unloaded);
            }
        }
        for newly_loaded in loaded_chunks.difference(loaded_before).cloned() {
            self.loader_entities.entry(newly_loaded).or_default().insert(uuid);
        }
        *loaded_before = loaded_chunks;
    }

    pub fn get_loaders(&self, position: &ChunkColumnPosition) -> Option<&HashSet<UUID>> {
        self.loader_entities.get(position)
    }

    pub fn get_loaded_chunks(&self) -> HashSet<ChunkColumnPosition> {
        self.loader_entities.keys().cloned().collect()
    }
}

/// World is the union of the map and entities.
/// World handles loaded chunks and entities.
/// It is responsible for notifying players of changes in the world.
pub struct World {
    map: WorldMap,
    entities: Entities,

    loading_manager: RwLock<WorldLoadingManager>,
    change_senders: RwLock<HashMap<UUID, MpscSender<WorldChange>>>,
}

impl World {
    pub fn new() -> World {
        World {
            map: WorldMap::new(4),
            entities: Entities::new(),
            loading_manager: RwLock::new(WorldLoadingManager::default()),
            change_senders: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_block(&self, position: BlockPosition) -> Option<BlockWithState> {
        Some(self.map.get_block(position).await)
    }

    pub async fn get_network_chunk(&self, position: ChunkPosition) -> Option<NetworkChunk> {
        self.map.get_network_chunk(position).await
    }

    pub async fn get_network_heightmap(&self, position: ChunkColumnPosition) -> Option<NbtTag> {
        self.map.get_network_heightmap(position).await
    }

    pub async fn set_block(&self, position: BlockPosition, block: BlockWithState) {
        self.map.set_block(position.clone(), block.clone()).await;
        self.notify(&position.chunk_column(), WorldChange::BlockChange(position, block)).await;
    }

    pub async fn add_loader(&self, uuid: UUID) -> MpscReceiver<WorldChange> {
        let (sender, receiver) = mpsc_channel(100);
        self.change_senders.write().await.insert(uuid, sender);
        receiver
    }

    pub async fn remove_loader(&self, uuid: UUID) {
        self.change_senders.write().await.remove(&uuid);
    }

    pub async fn update_loaded_chunks(&self, uuid: UUID, loaded_chunks: HashSet<ChunkColumnPosition>) {
        let mut loading_manager = self.loading_manager.write().await;
        let loaded_chunks_before = loading_manager.get_loaded_chunks();
        loading_manager.update_loaded_chunks(uuid, loaded_chunks);
        let loaded_chunks_after = loading_manager.get_loaded_chunks();
        let newly_loaded_chunks = loaded_chunks_after.difference(&loaded_chunks_before);
        let just_unloaded_chunks = loaded_chunks_before.difference(&loaded_chunks_after);
        drop(loading_manager);
        for newly_loaded_chunk in newly_loaded_chunks {
            self.map.load(newly_loaded_chunk.clone()).await;
        }
        for just_unloaded_chunk in just_unloaded_chunks {
            self.map.unload(just_unloaded_chunk.clone()).await;
        }
    }

    async fn notify(&self, position: &ChunkColumnPosition, change: WorldChange) {
        let loading_manager = self.loading_manager.read().await;
        let mut senders = self.change_senders.write().await;
        let Some(loaders) = loading_manager.get_loaders(position) else {return};
        for loader in loaders {
            if let Some(sender) = senders.get_mut(loader) {
                let _ = sender.send(change.clone()).await;
            }
        }
    }
}
