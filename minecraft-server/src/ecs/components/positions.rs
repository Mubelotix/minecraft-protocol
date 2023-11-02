use crate::prelude::*;

#[is_component]
pub struct PositionComponent {
    position: Position,
}

/*
impl Entities {
    pub async fn get_position(&self, id: Eid) -> Option<PositionComponent> {
        self.position_components.read().await.get(&id).cloned()
    }

    pub async fn set_position(&self, id: Eid, position: PositionComponent) -> Option<()> {
        let mut position_components = self.position_components.write().await;
        position_components.insert(id, position);
        Some(())
    }
}

impl PositionComponent {
    pub async fn set_position(&mut self, position: Position) -> Option<()> {
        self.position = position;
        Some(())
    }

    pub async fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub async fn move_entity(&mut self, delta: Position) -> Option<()> {
        self.position += delta;
        Some(())
    }
}

#[cfg(test)]
mod tests {
    
    #[tokio::test]
    async fn change_chunk() {
        use super::tags::Tag;
        let entities = crate::ecs::Entities::new();
        entities.create_entity_with_tag(Tag::Player).await.unwrap();

    }
}
*/