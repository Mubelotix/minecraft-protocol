use crate::{prelude::*, entities::AnyEntity};
use minecraft_protocol::packets::UUID;
use super::tags::Tag;

pub type Eid = u32;

pub struct Entities {    
    pub entities: RwLock<HashMap<Eid, AnyEntity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    
    // TODO: pub entities_by_tag: RwLock<HashMap<Tag, HashSet<Eid>>>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            entities: RwLock::new(HashMap::new()),
            chunks: RwLock::new(HashMap::new()),
            uuids: RwLock::new(HashMap::new()),
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

    /// Remove an entity
    pub async fn remove_entity(&self, eid: Eid) -> Option<AnyEntity> {
        self.entities.write().await.remove(&eid)
    }
}
