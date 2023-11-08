use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Clone)]
pub struct PositionComponent {
    position: Position,
}

impl Entities {
    pub async fn get_position(&self, id: Eid) -> Option<PositionComponent> {
        // we don't check if the entity has a position component because the hashmap will return None if the entity doesn't have it
        //self.position_components.read().await.get(&id).cloned()
        unimplemented!()
    }

    pub async fn set_position(&self, id: Eid, position: PositionComponent) -> Option<()> {        
//        let new_chunk = position.get_chunk();
//        let mut position_components = self.position_components.write().await;
//        let old_chunk = position_components.get(&id).cloned();
//        
//        // Update the position component
//        let pos = position_components.get_mut(&id)?;
//        *pos = position;
//        drop(position_components);
//
//        if let Some(old_chunk) = old_chunk {
//            let old_chunk = old_chunk.get_chunk();
//            if old_chunk != new_chunk {
//                // Remove the entity from the old chunk
//                let mut chunks = self.chunks.write().await;
//                let old_chunk_entities = chunks.get_mut(&old_chunk)?;
//                old_chunk_entities.remove(&id);
//
//                // Add the entity to the new chunk
//                let new_chunk_entities = chunks.get_mut(&new_chunk)?;
//                new_chunk_entities.insert(id); 
//                
//            }
//        } else {
//            // Add the entity to the new chunk
//            let mut chunks = self.chunks.write().await;
//            let new_chunk_entities = chunks.get_mut(&new_chunk)?;
//            new_chunk_entities.insert(id);
//        }
//        
//        Some(())
        unimplemented!()
    }
}

impl PositionComponent {
    pub async fn set_position(&mut self, position: Position) -> Option<()> {
        self.position = position;
        Some(())
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn move_entity(&mut self, delta: Position) {
        self.position += delta;
    }

    pub fn get_chunk(&self) -> ChunkPosition {
        self.position.chunk()
    }
}
