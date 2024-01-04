use std::collections::{HashMap, HashSet};
use crate::*;
use minecraft_protocol::packets::UUID;
use tokio::sync::RwLock;

pub struct Entities {
    eid_counter: std::sync::atomic::AtomicU32,
    uuid_counter: std::sync::atomic::AtomicU64, 
    tasks: RwLock<HashMap<Eid, EntityTask>>,
    entities: RwLock<HashMap<Eid, AnyEntity>>,
    change_set: RwLock<EntityChangeSet>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    chunks: RwLock<HashMap<ChunkColumnPosition, HashSet<Eid>>>,
    uuids: RwLock<HashMap<UUID, Eid>>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            eid_counter: std::sync::atomic::AtomicU32::new(0),
            uuid_counter: std::sync::atomic::AtomicU64::new(0),
            tasks: RwLock::new(HashMap::new()),
            entities: RwLock::new(HashMap::new()),
            change_set: RwLock::new(HashMap::new()),
            chunks: RwLock::new(HashMap::new()),
            uuids: RwLock::new(HashMap::new()),
        }
    }

    /// Observe an entity through a closure
    pub(super) async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.read().await.get(&eid).map(observer)
    }

    /// Observe entities in a chunk through a closure
    /// That closure will be applied to each entity, and the results will be returned in a vector
    pub(super) async fn observe_entities<R>(&self, chunk: ChunkColumnPosition, mut observer: impl FnMut(&AnyEntity, Eid) -> Option<R>) -> Vec<R> {
        let entities = self.entities.read().await;
        let chunks = self.chunks.read().await;
        let Some(eids) = chunks.get(&chunk) else {return Vec::new()};
        let mut results = Vec::with_capacity(eids.len());
        for eid in eids {
            if let Some(entity) = entities.get(eid) {
                if let Some(r) = observer(entity, *eid) {
                    results.push(r);
                }
            }
        }
        results
    }

    // TODO don't return [EntityChanges]
    /// Mutate an entity through a closure
    pub(super) async fn mutate_entity<R>(&self, eid: Eid, mutator: impl FnOnce(&mut AnyEntity) -> R) -> Option<(R, EntityChanges)> {
        let mut entities = self.entities.write().await;

        if let Some(entity) = entities.get_mut(&eid) {
            let prev_position = entity.as_entity().position.clone();
            let prev_velocity = entity.as_entity().velocity.clone();
            let prev_pitch = entity.as_entity().pitch;
            let prev_yaw = entity.as_entity().yaw;
            let r = mutator(entity);
            let mut changes = EntityChanges::other();
            if prev_velocity != entity.as_entity().velocity {
                changes += EntityChanges::velocity();
            }
            if prev_pitch != entity.as_entity().pitch || prev_yaw != entity.as_entity().yaw {
                changes += EntityChanges::pitch();
            }
            if prev_position != entity.as_entity().position {
                changes += EntityChanges::position();
                let old_chunk = prev_position.chunk_column();
                let new_chunk = entity.as_entity().position.chunk_column();
                drop(entities);
                let mut chunks = self.chunks.write().await;
                chunks.entry(old_chunk).and_modify(|set| { set.remove(&eid); }); // TODO: ensure it gets removed
                chunks.entry(new_chunk).or_insert(HashSet::new()).insert(eid);
            }
            *self.change_set.write().await.entry(eid).or_default() += changes;
            Some((r, changes))
        } else {
            None
        }
    }

    // TODO: Since we lock tasks it makes it impossible for an entity task to itself call this function
    // It would be resolved if we had a temporary task buffer that would be added only on Ecs::tick
    pub(super) async fn spawn_entity<E>(&self, entity: AnyEntity, world: &'static World, receiver: BroadcastReceiver<ServerMessage>) -> (Eid, UUID)
        where AnyEntity: TryAsEntityRef<E>, Handler<E>: EntityExt
    {
        let task = entity.init_task().await;
        let eid = self.eid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let uuid = self.uuid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as u128;
        let mut tasks = self.tasks.write().await;
        let mut entities = self.entities.write().await;
        let mut chunks = self.chunks.write().await;
        let mut uuids = self.uuids.write().await;
        chunks.entry(entity.as_entity().position.chunk_column()).or_insert(HashSet::new()).insert(eid);
        entities.insert(eid, entity);
        if let Some(task) = task {
            tasks.insert(eid, task);
        }
        uuids.insert(uuid, eid);
        drop(entities);
        drop(tasks);
        drop(chunks);
        drop(uuids);
        let h = Handler::<E>::assume(eid, world);
        h.init(receiver).await;
        (eid, uuid)
    }

    /// Remove an entity
    pub(super) async fn remove_entity(&self, eid: Eid) -> Option<AnyEntity> {
        let entity = self.entities.write().await.remove(&eid);
        let mut chunks = self.chunks.write().await;
        chunks.values_mut().for_each(|set| { set.remove(&eid); });
        chunks.retain(|_,v| !v.is_empty());
        drop(chunks);
        self.uuids.write().await.retain(|_,v| *v != eid);
        entity
    }

    pub(super) async fn tick(&self, world: &'static World) {
        let entity_change_set = std::mem::take(&mut *self.change_set.write().await);
        let mut tasks = self.tasks.write().await;
        for (eid, task) in tasks.iter_mut() {
            let h = Handler::<Entity>::assume(*eid, world);
            task.tick(h, &entity_change_set).await;
        }
    }
}
