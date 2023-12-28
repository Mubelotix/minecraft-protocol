use crate::prelude::*;

mod change;
pub use change::*;
mod loading_manager;
use loading_manager::*;
mod map;
use map::*;
mod light;
pub use light::*;
mod ecs;
use ecs::*;
mod collisions;
pub use collisions::*;

/// World is the union of the map and entities.
/// World handles loaded chunks and entities.
/// It is responsible for notifying players of changes in the world.
pub struct World {
    map: WorldMap,
    entities: Entities,

    loading_manager: RwLock<WorldLoadingManager>,
    change_senders: RwLock<HashMap<UUID, MpscSender<WorldChange>>>, // TODO: Add a way to select events you want to subscribe to
    receiver: BroadcastReceiver<ServerMessage>,
}

impl World {
    pub fn new(receiver: BroadcastReceiver<ServerMessage>) -> World {
        World {
            map: WorldMap::new(4),
            entities: Entities::new(),
            loading_manager: RwLock::new(WorldLoadingManager::default()),
            change_senders: RwLock::new(HashMap::new()),
            receiver,
        }
    }

    pub async fn get_block(&self, position: BlockPosition) -> Option<BlockWithState> {
        Some(self.map.get_block(position).await)
    }

    pub async fn set_block(&'static self, position: BlockPosition, block: BlockWithState) {
        self.map.set_block(position.clone(), block.clone()).await;
        self.notify(&position.chunk_column(), WorldChange::Block(position, block)).await;
    }

    pub async fn try_move(&self, object: &CollisionShape, movement: &Translation) -> Translation {
        self.map.try_move(object, movement).await
    }

    pub async fn add_loader(&self, uuid: UUID) -> MpscReceiver<WorldChange> {
        let (sender, receiver) = mpsc_channel(100);
        self.change_senders.write().await.insert(uuid, sender);
        receiver
    }

    pub async fn remove_loader(&self, uuid: UUID) {
        self.change_senders.write().await.remove(&uuid);
    }

    pub async fn ensure_loaded_chunks(&'static self, uuid: UUID, loaded_chunks: HashSet<ChunkColumnPosition>) {
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

    pub async fn spawn_entity<E>(&'static self, entity: AnyEntity) -> Eid
        where AnyEntity: TryAsEntityRef<E>, Handler<E>: EntityExt
    {
        let position = entity.as_entity().position.clone();
        let velocity = entity.as_entity().velocity.clone();
        let ty = entity.to_network().unwrap(); // TODO: error handling
        let pitch = entity.as_entity().pitch;
        let yaw = entity.as_entity().yaw;
        let head_yaw = entity.as_other::<LivingEntity>().map(|e| e.head_yaw).unwrap_or(0.0);
        let (eid, uuid) = self.entities.spawn_entity::<E>(entity, self, self.receiver.resubscribe()).await;
        self.notify(&position.chunk_column(), WorldChange::EntitySpawned {
            eid,
            uuid,
            ty,
            position,
            pitch,
            yaw,
            head_yaw,
            data: 0,
            velocity,
            metadata: (),
        }).await;
        eid
    }

    pub async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.observe_entity(eid, observer).await
    }

    pub async fn observe_entities<R>(&self, chunk: ChunkColumnPosition, observer: impl FnMut(&AnyEntity) -> Option<R>) -> Vec<R> {
        self.entities.observe_entities(chunk, observer).await
    }

    // TODO: add version that doesn't notify modified entity
    pub async fn mutate_entity<R>(&self, eid: Eid, mutator: impl FnOnce(&mut AnyEntity) -> (R, EntityChanges)) -> Option<R> {
        // TODO: change events
        match self.entities.mutate_entity(eid, mutator).await {
            Some((r, changes)) => {
                // TODO: make only one lookup and group into a single message with optional fields
                let position = self.entities.observe_entity(eid, |e| e.as_entity().position.clone()).await?;
                if changes.position_changed() {
                    self.notify(&position.chunk_column(), WorldChange::EntityPosition {
                        eid,
                        position: position.clone(),
                    }).await;
                }
                if changes.velocity_changed() {
                    let velocity = self.entities.observe_entity(eid, |e| e.as_entity().velocity.clone()).await?;
                    self.notify(&position.chunk_column(), WorldChange::EntityVelocity {
                        eid,
                        velocity,
                    }).await;
                }
                if changes.pitch_changed() {
                    let (pitch, yaw, head_yaw) = self.entities.observe_entity(eid, |e| (e.as_entity().pitch, e.as_entity().yaw, e.as_other::<LivingEntity>().map(|e| e.head_yaw).unwrap_or(0.0))).await?;
                    self.notify(&position.chunk_column(), WorldChange::EntityPitch {
                        eid,
                        pitch,
                        yaw,
                        head_yaw,
                    }).await;
                }
                if changes.metadata_changed() {
                    todo!()
                }
                Some(r)
            },
            None => None,
        }
    }

    async fn notify(&self, position: &ChunkColumnPosition, change: WorldChange) {
        let loading_manager = self.loading_manager.read().await;
        let mut senders = self.change_senders.write().await;
        let Some(loaders) = loading_manager.get_loaders(position) else {return};
        for loader in loaders {
            if let Some(sender) = senders.get_mut(loader) {
                let _ = sender.try_send(change.clone());
            }
        }
    }

    #[cfg_attr(feature = "trace", instrument(skip(self)))]
    pub async fn get_network_chunk_column_data<'a>(&self, position: ChunkColumnPosition) -> Option<Vec<u8>> {
        self.map.get_network_chunk_column_data(position).await
    }

    pub async fn get_light_level(&'static self, position: BlockPosition) -> u8 {
        LightManager::new(&self.map).get_light_level(LightPosition::from(position)).await
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::error::TryRecvError;

    #[tokio::test]
    async fn test_world_notifications() {
        let world = Box::leak(Box::new(World::new(broadcast_channel(100).1)));

        let mut receiver1 = world.add_loader(1).await;
        let mut receiver2 = world.add_loader(2).await;
        world.ensure_loaded_chunks(1, vec![ChunkColumnPosition{cx: 0, cz: 0}].into_iter().collect()).await;
        world.ensure_loaded_chunks(2, vec![ChunkColumnPosition{cx: 1, cz: 1}].into_iter().collect()).await;

        world.set_block(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air).await;
        assert!(matches!(receiver1.try_recv(), Ok(WorldChange::Block(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air))));
        assert!(matches!(receiver2.try_recv(), Err(TryRecvError::Empty)));
    }
}
