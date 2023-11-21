use crate::prelude::*;

mod change;
pub use change::*;
mod map;
use map::*;
mod ecs;
pub use ecs::*;
mod collisions;
pub use collisions::*;

/// World is the union of the map and entities.
/// World handles loaded chunks and entities.
/// It is responsible for notifying players of changes in the world.
pub struct World {
    map: WorldMap,
    entities: Entities,

    world_observer_manager: WorldObserverManager,
}

impl World {
    pub fn new() -> World {
        World {
            map: WorldMap::new(4),
            entities: Entities::new(),
            world_observer_manager: WorldObserverManager::new(),
        }
    }

    pub async fn get_block(&self, position: BlockPosition) -> Option<BlockWithState> {
        Some(self.map.get_block(position).await)
    }

    pub async fn get_network_chunk(&self, position: ChunkPosition) -> Option<NetworkChunk> {
        self.map.get_network_chunk(position).await
    }

    pub async fn set_block(&self, position: BlockPosition, block: BlockWithState) {
        self.map.set_block(position.clone(), block.clone()).await;
        self.world_observer_manager.notify_block_change(position.clone(), block.clone()).await;
    }

    pub async fn try_move(&self, object: &CollisionShape, movement: &Translation) -> Translation {
        self.map.try_move(object, movement).await
    }

    pub fn new_world_observer(&'static self, eid: Eid) -> WorldObserverBuilder {
        WorldObserverBuilder::new(eid, &self.world_observer_manager)
    }

    pub async fn tick(&self, tick_id: usize) {
        self.world_observer_manager.notify_tick(tick_id).await;
    }

    pub async fn reserve_eid(&self) -> ReservedEid {
        self.entities.reserve_eid().await
    }

    pub async fn spawn_entity_with_reserved_eid<E>(&'static self, eid: ReservedEid, entity: AnyEntity) -> Eid
        where AnyEntity: TryAsEntityRef<E>, Handler<E>: EntityExt
    {
        let position = entity.as_entity().position.clone();
        let velocity = entity.as_entity().velocity.clone();
        let ty = entity.to_network().unwrap(); // TODO: error handling
        let pitch = entity.as_entity().pitch;
        let yaw = entity.as_entity().yaw;
        let head_yaw = entity.as_other::<LivingEntity>().map(|e| e.head_yaw).unwrap_or(0.0);
        let (eid, uuid) = self.entities.spawn_entity::<E>(eid, entity, self).await;
        let data = 0; // TODO: Support entity data
        self.world_observer_manager.notify_entity_spawned(eid, uuid, ty, position, pitch, yaw, head_yaw, data, velocity).await;
        eid
    }

    pub async fn spawn_entity<E>(&'static self, entity: AnyEntity) -> Eid
        where AnyEntity: TryAsEntityRef<E>, Handler<E>: EntityExt
    {
        let eid = self.reserve_eid().await;
        self.spawn_entity_with_reserved_eid::<E>(eid, entity).await
    }

    pub async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.observe_entity(eid, observer).await
    }

    pub async fn observe_entities<R>(&self, chunk: ChunkColumnPosition, observer: impl FnMut(&AnyEntity) -> Option<R>) -> Vec<R> {
        self.entities.observe_entities(chunk, observer).await
    }

    // TODO: add version that doesn't notify modified entity
    pub async fn mutate_entity<R>(&'static self, eid: Eid, mutator: impl FnOnce(&mut AnyEntity) -> (R, EntityChanges)) -> Option<R> {
        // TODO: change events
        let previous_position = self.entities.observe_entity(eid, |e| e.as_entity().position.clone()).await?;
        match self.entities.mutate_entity(eid, mutator, &self.world_observer_manager).await {
            Some((r, changes)) => {
                // TODO: make only one lookup and group into a single message with optional fields
                let position = self.entities.observe_entity(eid, |e| e.as_entity().position.clone()).await?;
                if changes.position_changed() {
                    self.world_observer_manager.notify_entity_moved(eid, previous_position, position.clone()).await;
                }
                if changes.velocity_changed() {
                    let velocity = self.entities.observe_entity(eid, |e| e.as_entity().velocity.clone()).await?;
                    self.world_observer_manager.notify_entity_velocity(eid, position.clone(), velocity).await;
                }
                if changes.pitch_changed() {
                    let (pitch, yaw, head_yaw) = self.entities.observe_entity(eid, |e| (e.as_entity().pitch, e.as_entity().yaw, e.as_other::<LivingEntity>().map(|e| e.head_yaw).unwrap_or(0.0))).await?;
                    self.world_observer_manager.notify_entity_pitch(eid, position.clone(), pitch, yaw, head_yaw).await;
                }
                if changes.metadata_changed() {
                    todo!()
                }
                Some(r)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc::error::TryRecvError;

    #[tokio::test]
    async fn test_world_notifications() {
        let world = Box::leak(Box::new(World::new()));

        let mut receiver1 = WorldObserverBuilder::new(1, &world.world_observer_manager).with_blocks_in_chunk(ChunkColumnPosition{cx: 0, cz: 0}).build().await;
        let mut receiver2 = WorldObserverBuilder::new(1, &world.world_observer_manager).with_blocks_in_chunk(ChunkColumnPosition{cx: 1, cz: 1}).build().await;

        world.set_block(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air).await;
        assert!(matches!(receiver1.try_recv(), Ok(WorldChange::Block(BlockPosition{x: 1, y: 1, z: 1}, BlockWithState::Air))));
        assert!(matches!(receiver2.try_recv(), Err(TryRecvError::Empty)));
    }
}
