use crate::prelude::*;

use super::tags::Tag;

pub type Eid = u32;

#[derive(Clone, PartialEq, Eq)]
pub struct Entity {
    /// The entity's unique ID. In the current world
    /// this is unique at any given time but may be
    /// reused when an entity is removed for another
    id: Eid,
    /// Components attached to this entity
    components: HashSet<Component>,
    /// The entity tag
    tag: Tag,
}


pub struct Entities {    
    pub entities: RwLock<HashMap<Eid, Entity>>,

    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entities_by_tag: RwLock<HashMap<Tag, HashSet<Eid>>>,

    pub health_components: RwLock<HashMap<Eid, HealthComponent>>,
    pub position_components: RwLock<HashMap<Eid, PositionComponent>>,
}


impl Entities {
    /// Query a specific entity
    pub async fn get_entity(&self, id: Eid) -> Option<Entity> {
        self.entities.read().await.get(&id).cloned()
    }

    /// Query an entity by his tag
    pub async fn get_entities_by_tag(&self, tag: Tag) -> HashSet<Eid> {
        self.entities_by_tag.read().await.get(&tag).cloned().unwrap_or_default()
    }

    /// Query an entity by his components
    /// We get all tags that have the components
    pub async fn get_entities_with(&self, components: HashSet<Component>) -> Vec<Entity> {
        // We get all tags that have the components
        let mut tags = Tag::get_tags_from_components(components.clone());
        // We get all entities that have the tags
        let mut entities = Vec::new();
        for tag in tags.drain() {
            let mut entities_with_tag = self.get_entities_by_tag(tag).await;
            for entity in entities_with_tag.drain() {
                // TODO: remove this check but add an error handling
                if let Some(entity) = self.get_entity(entity).await {
                    entities.push(entity);
                }
            }
        }
        entities
    }
}
