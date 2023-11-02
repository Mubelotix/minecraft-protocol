use std::future::Future;

use crate::prelude::*;

use super::tags::Tag;


#[derive(Clone, PartialEq, Eq)]
pub struct Entity {
    /// The entity's unique ID. In the current world
    /// this is unique at any given time but may be
    /// reused when an entity is removed for another
    id: Eid,
    /// Components attached to this entity
    components: HashSet<ComponentId>,
    /// The entity tag
    tag: Tag,
}


pub struct Entities {    
    pub entities: RwLock<HashMap<Eid, Entity>>,
    entity_count: RwLock<Eid>,
    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entities_by_tag: RwLock<HashMap<Tag, HashSet<Eid>>>,

    pub component_storage: ComponentStorage,
}


impl Entities {
    pub fn new() -> Self {
        Self {
            entities: RwLock::new(HashMap::new()),
            entity_count: RwLock::new(0),
            chunks: RwLock::new(HashMap::new()),
            uuids: RwLock::new(HashMap::new()),
            entities_by_tag: RwLock::new(HashMap::new()),
            component_storage: ComponentStorage::default(),
        
        }
    }

    async fn inc_eid(&self) -> Eid {
        let mut entity_count = self.entity_count.write().await;
        *entity_count += 1;
        *entity_count
    }

    pub fn attach_component<T: Component>(&self, id: Eid, component: T) -> impl Future<Output = Option<()>> + '_  {
        self.component_storage.attach_component::<T>(id, component)
    }

    pub fn get_component<T: Component + Clone + 'static>(&self, id: Eid, component_id: ComponentId) -> impl Future<Output = Option<T>> + '_ {
        self.component_storage.get_component(id, component_id)
    }

    pub fn update_component<T: Component>(&self, id: Eid, component: T) -> impl Future<Output = Option<()>> + '_ {
        self.component_storage.update_component(id, component)
    }

    pub fn remove_component(&self, id: Eid, component_id: ComponentId) -> impl Future<Output = Option<()>> + '_ {
        self.component_storage.remove_component(id, component_id)
    }

    pub fn get_entities_with(&self, component_id: ComponentId) -> impl Future<Output = HashSet<Eid>> + '_{
        self.component_storage.get_entities_with(component_id)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_basic_ecs() {
        
    }
}
