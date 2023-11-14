use crate::prelude::*;

mod change_event;
pub use change_event::*;
mod loading_manager;
use loading_manager::*;
mod map;
use map::*;
mod light;
use light::*;
/// World is the union of the map and entities.
/// World handles loaded chunks and entities.
/// It is responsible for notifying players of changes in the world.
pub struct World {
    map: WorldMap,
    entities: Entities,

    loading_manager: RwLock<WorldLoadingManager>,
    change_senders: RwLock<HashMap<UUID, MpscSender<WorldChange>>>, // TODO: Add a way to select events you want to subscribe to
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

    pub async fn get_network_chunk_column_data(&self, position: ChunkColumnPosition) -> Option<Vec<u8>> {
        self.map.get_network_chunk_column_data(position).await
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::error::TryRecvError;

    #[tokio::test]
    async fn test_world_notifications() {
        let world = World::new();

        let mut receiver1 = world.add_loader(1).await;
        let mut receiver2 = world.add_loader(2).await;
        world.update_loaded_chunks(1, vec![ChunkColumnPosition{cx: 0, cz: 0}].into_iter().collect()).await;
        world.update_loaded_chunks(2, vec![ChunkColumnPosition{cx: 1, cz: 1}].into_iter().collect()).await;

        world.set_block(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air).await;
        assert!(matches!(receiver1.try_recv(), Ok(WorldChange::BlockChange(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air))));
        assert!(matches!(receiver2.try_recv(), Err(TryRecvError::Empty)));
    }
}
