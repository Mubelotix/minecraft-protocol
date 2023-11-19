use std::collections::{HashMap, HashSet};
use crate::*;
use minecraft_protocol::packets::UUID;
use tokio::sync::RwLock;

pub type EntityTask = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>;
pub type EntityTaskHandle = tokio::task::JoinHandle<()>;

pub struct Entities {
    eid_counter: std::sync::atomic::AtomicU32,
    uuid_counter: std::sync::atomic::AtomicU64, 
    pub entities: RwLock<HashMap<Eid, AnyEntity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkColumnPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entity_tasks: RwLock<HashMap<Eid, HashMap<&'static str, EntityTaskHandle>>>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            eid_counter: std::sync::atomic::AtomicU32::new(0),
            uuid_counter: std::sync::atomic::AtomicU64::new(0),
            entities: RwLock::new(HashMap::new()),
            chunks: RwLock::new(HashMap::new()),
            uuids: RwLock::new(HashMap::new()),
            entity_tasks: RwLock::new(HashMap::new()),
        }
    }

    /// Observe an entity through a closure
    pub(super) async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.read().await.get(&eid).map(observer)
    }

    /// Observe entities in a chunk through a closure
    /// That closure will be applied to each entity, and the results will be returned in a vector
    pub(super) async fn observe_entities<R>(&self, chunk: ChunkColumnPosition, mut observer: impl FnMut(&AnyEntity) -> Option<R>) -> Vec<R> {
        let entities = self.entities.read().await;
        let chunks = self.chunks.read().await;
        let Some(eids) = chunks.get(&chunk) else {return Vec::new()};
        let mut results = Vec::with_capacity(eids.len());
        for eid in eids {
            if let Some(entity) = entities.get(eid) {
                if let Some(r) = observer(entity) {
                    results.push(r);
                }
            }
        }
        results
    }

    /// Mutate an entity through a closure
    pub(super) async fn mutate_entity<R>(
        &self,
        eid: Eid,
        mutator: impl FnOnce(&mut AnyEntity) -> (R, EntityChanges),
        observer_manager: &'static WorldObserverManager
    ) -> Option<(R, EntityChanges)> {
        let mut entities = self.entities.write().await;

        if let Some(entity) = entities.get_mut(&eid) {
            let prev_position = entity.as_entity().position.clone();
            let (r, changes) = mutator(entity);
            let old_chunk = prev_position.chunk_column();
            let new_chunk = entity.as_entity().position.chunk_column();
            let e = entity.as_entity();
            if changes.position_changed() {
                observer_manager.notify_entity_moved(eid, prev_position, e.position.clone()).await;
            }
            if changes.velocity_changed() {
                observer_manager.notify_entity_velocity(eid, e.position.clone(), e.velocity.clone()).await;
            }
            if changes.pitch_changed() {
                observer_manager.notify_entity_pitch(eid, e.position.clone(), e.pitch, e.yaw, entity.as_other::<LivingEntity>().map(|e| e.head_yaw).unwrap_or(0.0)).await;
            }
            if changes.metadata_changed() {
                // TODO: Support metadata changes
                // observer_manager.notify_metadata_change(eid, e.position.clone(), e.metadata.clone()).await;
            }
            drop(entities);
            if old_chunk != new_chunk {
                let mut chunks = self.chunks.write().await;
                chunks.entry(old_chunk).and_modify(|set| { set.remove(&eid); }); // TODO: ensure it gets removed
                chunks.entry(new_chunk).or_insert(HashSet::new()).insert(eid);
            }
            Some((r, changes))
        } else {
            None
        }
    }

    pub(super) async fn spawn_entity<E>(&self, entity: AnyEntity, world: &'static World) -> (Eid, UUID)
        where AnyEntity: TryAsEntityRef<E>, Handler<E>: EntityExt
    {
        let eid = self.eid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let uuid = self.uuid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as u128;
        let mut entities = self.entities.write().await;
        let mut chunks = self.chunks.write().await;
        let mut uuids = self.uuids.write().await;
        chunks.entry(entity.as_entity().position.chunk_column()).or_insert(HashSet::new()).insert(eid);
        entities.insert(eid, entity);
        uuids.insert(uuid, eid);
        drop(entities);
        drop(chunks);
        drop(uuids);
        let h = Handler::<E>::assume(eid, world);
        h.init().await;
        (eid, uuid)
    }

    pub(super) async fn insert_entity_task(&self, eid: Eid, name: &'static str, handle: EntityTaskHandle) {
        let mut entity_tasks = self.entity_tasks.write().await;
        let old = entity_tasks.entry(eid).or_insert(HashMap::new()).insert(name, handle);
        if let Some(old) = old {
            old.abort();
        }
    }

    /// Remove an entity
    pub(super) async fn remove_entity(&self, eid: Eid, observer_manager: &'static WorldObserverManager) -> Option<AnyEntity> {
        let entity = self.entities.write().await.remove(&eid);
        let mut chunks = self.chunks.write().await;
        chunks.values_mut().for_each(|set| { set.remove(&eid); });
        chunks.retain(|_,v| !v.is_empty());
        drop(chunks);
        self.uuids.write().await.retain(|_,v| *v != eid);
        self.entity_tasks.write().await.remove(&eid);
        if let Some(entity) = &entity {
            observer_manager.notify_entity_dispawned(eid, entity.as_entity().position.clone()).await;
        }
        entity
    }
}

impl<T> Handler<T> where AnyEntity: TryAsEntityRef<T> {
    pub async fn insert_task(&self, name: &'static str, handle: EntityTaskHandle) {
        self.world.entities.insert_entity_task(self.eid, name, handle).await;
    }
}
