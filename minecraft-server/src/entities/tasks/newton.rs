use crate::CollisionShape;

use super::*;

pub struct NewtonTask {
    width: f64,
    height: f64,
}

impl NewtonTask {
    pub async fn init(entity: &AnyEntity) -> Option<NewtonTask> {
        let network_entity = entity.to_network();
    
        let (width, height) = match network_entity {
            Some(network_entity) => (network_entity.width() as f64, network_entity.height() as f64),
            None => {
                warn!("Entity has no network entity");
                return None;
            }
        };
        
        Some(NewtonTask {
            width,
            height,
        })
    }

    pub async fn tick(&mut self, h: Handler<Entity>) {
        // Get data from entity
        let Some((mut position, mut velocity)) = h.observe_any(|any_entity| {
            let entity = any_entity.as_entity();
            (entity.position.clone(), entity.velocity.clone())
        }).await else { return; };

        // Apply velocity and collisions
        let mut changes = EntityChanges::nothing();
        let mut new_velocity = velocity.clone();
        new_velocity.y -= 9.81/20.0;
        let bounding_box = CollisionShape {
            x1: position.x - self.width/2.0,
            y1: position.y,
            z1: position.z - self.width/2.0,
            x2: position.x + self.width/2.0,
            y2: position.y + self.height,
            z2: position.z + self.width/2.0,
        };
        let new_velocity = h.world.try_move(&bounding_box, &new_velocity).await;
        if velocity.x != new_velocity.x {
            velocity.x = 0.0;
            changes += EntityChanges::velocity();
        }
        if velocity.y != new_velocity.y {
            velocity.y = 0.0;
            changes += EntityChanges::velocity();
        }
        if velocity.z != new_velocity.z {
            velocity.z = 0.0;
            changes += EntityChanges::velocity();
        }
        if !new_velocity.is_zero() {
            changes += EntityChanges::position();
            position += new_velocity;
        }

        // TODO(feat): Apply air resistance to x and z velocity
        // Keep in mind that velocity shouldn't flicker when constantly kept up by another task but slowed down in this task

        // Mutate entity
        // TODO(correctness): Before modifying entity values, we should ensure the original values we based the changes on are still the same
        if changes.nothing_changed() {
            return;
        }
        h.mutate(|entity| {
            let entity = entity.get_entity_mut();
            entity.velocity = velocity;
            entity.position = position;
            ((), changes)
        }).await;
    }
}
