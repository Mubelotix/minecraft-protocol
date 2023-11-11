use std::collections::{HashMap, HashSet};
use crate::*;
use minecraft_protocol::packets::UUID;
use tokio::sync::RwLock;

pub type EntityTask = Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>;

pub struct Entities {
    eid_counter: std::sync::atomic::AtomicU32,
    uuid_counter: std::sync::atomic::AtomicU64, 
    pub entities: RwLock<HashMap<Eid, AnyEntity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entity_tasks: RwLock<HashMap<Eid, HashMap<&'static str, EntityTask>>>,
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
    pub async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.read().await.get(&eid).map(observer)
    }

    /// Mutate an entity through a closure
    pub async fn mutate_entity<R>(&self, eid: Eid, mutator: impl FnOnce(&mut AnyEntity) -> R) -> Option<R> {
        let mut entities = self.entities.write().await;

        if let Some(entity) = entities.get_mut(&eid) {
            let prev_position = entity.as_entity().position.clone();
            let r = mutator(entity);
            if prev_position != entity.as_entity().position {
                let old_chunk = prev_position.chunk();
                let new_chunk = entity.as_entity().position.chunk();
                drop(entities);
                let mut chunks = self.chunks.write().await;
                chunks.get_mut(&old_chunk).unwrap().remove(&eid);
                chunks.get_mut(&new_chunk).unwrap().insert(eid);
            }
            Some(r)
        } else {
            None
        }
    }

    pub async fn spawn_entity(&self, entity: AnyEntity) -> (Eid, UUID) {
        let eid = self.eid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let uuid = self.uuid_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as u128;
        let mut entities = self.entities.write().await;
        let mut chunks = self.chunks.write().await;
        let mut uuids = self.uuids.write().await;
        chunks.entry(entity.as_entity().position.chunk()).or_insert(HashSet::new()).insert(eid);
        entities.insert(eid, entity);
        uuids.insert(uuid, eid);
        (eid, uuid)
    }

    /// Remove an entity
    pub async fn remove_entity(&self, eid: Eid) -> Option<AnyEntity> {
        let entity = self.entities.write().await.remove(&eid);
        let mut chunks = self.chunks.write().await;
        chunks.values_mut().for_each(|set| { set.remove(&eid); });
        chunks.retain(|_,v| !v.is_empty());
        drop(chunks);
        self.uuids.write().await.retain(|_,v| *v != eid);
        self.entity_tasks.write().await.remove(&eid);
        entity
    }
}
