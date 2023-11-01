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

    /// Returns true if the entity is dead.
    pub async fn is_dead(&self, id: Eid) -> Option<bool> {
        self.get_health(id).await.map(|health| health.health <= 0.0)
    }

    /// Heals the entity by the given amount.
    pub async fn heal(&self, id: Eid, amount: f32) -> Option<()> {
        let mut health_components = self.health_components.write().await;
        let health = health_components.get_mut(&id)?;
        health.health = (health.health + amount).min(health.max_health);
        Some(())
    }

    /// Fully heals the entity.
    pub async fn full_heal(&self, id: Eid) -> Option<()> {
        let mut health_components = self.health_components.write().await;
        let health = health_components.get_mut(&id)?;
        health.health = health.max_health;
        Some(())
    }

    /// Damages the entity by the given amount.
    pub async fn damage(&self, id: Eid, amount: f32) -> Option<()> {
        let mut health_components = self.health_components.write().await;
        let health = health_components.get_mut(&id)?;
        health.health -= amount;
        health.health = health.health.max(0.0);
        Some(())
    }
}
