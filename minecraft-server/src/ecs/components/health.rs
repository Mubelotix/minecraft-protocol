use crate::prelude::*;

#[derive(Clone)]
pub struct HealthComponent {
    pub health: f32,
    pub max_health: f32,
}

impl Entities {
    pub async fn get_health(&self, id: Eid) -> Option<HealthComponent> {
        self.health_components.read().await.get(&id).cloned()
    }
}
