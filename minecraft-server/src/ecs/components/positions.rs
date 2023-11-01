use crate::prelude::*;

#[derive(Clone)]
pub struct PositionComponent {
    position: Position,
}

impl Entities {
    pub async fn get_position(&self, id: Eid) -> Option<PositionComponent> {
        self.position_components.read().await.get(&id).cloned()
    }
}