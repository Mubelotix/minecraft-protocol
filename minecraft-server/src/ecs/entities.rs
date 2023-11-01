use crate::prelude::*;

pub type Eid = u32;

#[derive(Clone)]
pub struct Entity {
    /// The entity's unique ID. In the current world
    /// this is unique at any given time but may be
    /// reused when an entity is removed for another
    id: Eid,
    /// Components attached to this entity
    components: HashSet<Components>
}

pub struct Entities {    
    entities: RwLock<HashMap<Eid, Entity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    uuids: RwLock<HashMap<UUID, Eid>>,

    health_components: RwLock<HashMap<Eid, HealthComponent>>,
    position_components: RwLock<HashMap<Eid, PositionComponent>>,
}

impl Entities {
    /// Query a specific entity
    pub async fn get_entity(&self, id: Eid) -> Option<Entity> {
        self.entities.read().await.get(&id).cloned()
    }

    
}
