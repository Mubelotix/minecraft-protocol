use crate::prelude::*;
use minecraft_ecs_macros::insert_components_fields;
use minecraft_protocol::packets::UUID;
use super::tags::Tag;


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

impl Entity {
    pub fn insert_component(&mut self, component: Component) {
        self.components.insert(component);
    }
}

#[insert_components_fields]
pub struct Entities {    
    pub entities: RwLock<HashMap<Eid, Entity>>,
    entity_count: RwLock<Eid>,
    /// A hashmap of chunk positions to get a list of entities in a chunk
    pub chunks: RwLock<HashMap<ChunkPosition, HashSet<Eid>>>,
    pub uuids: RwLock<HashMap<UUID, Eid>>,
    pub entities_by_tag: RwLock<HashMap<Tag, HashSet<Eid>>>,
}

impl Entities {
    async fn inc_eid(&self) -> Eid {
        let mut entity_count = self.entity_count.write().await;
        *entity_count += 1;
        *entity_count
    }

    /// Create a new entity
    pub async fn create_entity_with_tag(&self, tag: Tag) -> Option<Eid> {
        let id = self.inc_eid().await;
        let entity = Entity {
            id,
            components: tag.clone().get_components().into_iter().collect(),
            tag: tag.clone(),
        };
        self.entities.write().await.insert(id, entity);
        self.entities_by_tag.write().await.entry(tag.clone()).or_default().insert(id);

        // Add the components of the tag
        for component in tag.get_components() {
            
        }

        Some(id)
    }

    /// Create a new entity with components
    pub async fn create_entity_with_components(&self, components: HashSet<Component>) -> Option<Eid> {
        let id = self.inc_eid().await;
        let tag = Tag::get_tags_from_components(components.clone()).into_iter().next()?;
        let entity = Entity {
            id,
            components: components.clone(),
            tag: tag.clone(),
        };
        self.entities.write().await.insert(id, entity);
        self.entities_by_tag.write().await.entry(tag).or_default().insert(id);
        Some(id)
    }
    
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

impl Entity {

}
