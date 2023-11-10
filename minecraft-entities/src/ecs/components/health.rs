use crate::prelude::*;

#[derive(Clone)]
pub struct HealthComponent {
    pub health: f32,
    pub max_health: f32,
}

impl HealthComponent {
    /// Returns true if the entity is dead.s
    pub async fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub async fn get_health(&self) -> f32{
        self.health
    }

    /// Heals the entity by the given amount.
    pub async fn heal(&mut self, amount: f32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    /// Fully heals the entity.
    pub async fn full_heal(&mut self) {
        self.health = self.max_health;
    }

    /// Damages the entity by the given amount.
    pub async fn damage(&mut self, amount: f32) -> Option<()> {
        self.health -= amount;
        self.health = self.health.max(0.0);
        Some(())
    }
}

impl Entities {
    /// Set the health of an entity.
    pub async fn set_health(&self, id: Eid, health: HealthComponent) -> Option<()> {
        //let mut health_components = self.health_components.write().await;
        //health_components.insert(id, health);
        //Some(())
        unimplemented!()
    }
    
    /// Get the health of an entity.
    pub async fn get_health(&self, id: Eid) -> Option<HealthComponent> {
        //self.health_components.read().await.get(&id).cloned()
        unimplemented!()
    }
}
