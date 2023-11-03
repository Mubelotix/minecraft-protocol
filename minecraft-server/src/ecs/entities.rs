use crate::{prelude::*, entities::AnyEntity};
use minecraft_protocol::packets::UUID;
use super::tags::Tag;

pub type Eid = u32;

pub struct Entities {    
    pub entities: RwLock<HashMap<Eid, AnyEntity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entities_by_tag: RwLock<HashMap<Tag, HashSet<Eid>>>,

    pub health_components: RwLock<HashMap<Eid, HealthComponent>>,
    pub position_components: RwLock<HashMap<Eid, PositionComponent>>,
}

impl Entities {
    /// Observe an entity through a closure
    pub async fn observe_entity<R>(&self, eid: Eid, observer: impl FnOnce(&AnyEntity) -> R) -> Option<R> {
        self.entities.read().await.get(&eid).map(observer)
    }

    /// Mutate an entity through a closure
    pub async fn mutate_entity<R>(&self, eid: Eid, mutator: impl FnOnce(&mut AnyEntity) -> R) -> Option<R> {
        self.entities.write().await.get_mut(&eid).map(mutator)
    }

    /// Remove an entity
    pub async fn remove_entity(&self, eid: Eid) -> Option<AnyEntity> {
        self.entities.write().await.remove(&eid)
    }
}
