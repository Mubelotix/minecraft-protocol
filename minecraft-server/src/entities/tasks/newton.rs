use crate::CollisionShape;

use super::*;

/// This task applies gravity and velocity to an entity.
/// It has minimal performance impact.
/// 
/// It can be used by other larger tasks.
/// In that case, other tasks should stick to modifying `x`, `z`, `vx`, `vz` and `vy`.
/// This task shall be called at the end of their tick.
/// See the [ZombieTask] for an example.
pub struct NewtonTask {
    width: f64,
    height: f64,
    on_ground: bool,
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
            on_ground: false,
        })
    }

    pub async fn tick(&mut self, h: Handler<Entity>, entity_change_set: &EntityChangeSet) {
        // If it was on ground before and hasn't moved since, skip the turn
        // TODO: detect if the ground is destroyed
        if self.on_ground && !entity_change_set.get(&h.eid).copied().unwrap_or_default().position_changed() {
            return;
        }

        // Get data from entity
        let Some((mut position, mut velocity)) = h.observe_any(|any_entity| {
            let entity = any_entity.as_entity();
            (entity.position.clone(), entity.velocity.clone())
        }).await else { return; };

        // Apply velocity and collisions
        velocity.y -= 9.81/20.0;
        let bounding_box = CollisionShape {
            x1: position.x - self.width/2.0,
            y1: position.y,
            z1: position.z - self.width/2.0,
            x2: position.x + self.width/2.0,
            y2: position.y + self.height,
            z2: position.z + self.width/2.0,
        };
        let new_velocity = h.world.try_move(&bounding_box, &velocity).await;
        self.on_ground = velocity.y < 0.0 && new_velocity.y >= 0.0;
        if !velocity.is_zero() {
            position += new_velocity.clone();
        }

        // TODO(feat): Apply air resistance to x and z velocity
        // Keep in mind that velocity shouldn't flicker when constantly kept up by another task but slowed down in this task

        // Mutate entity
        // TODO(correctness): Before modifying entity values, we should ensure the original values we based the changes on are still the same
        h.mutate(|entity| {
            let entity = entity.get_entity_mut();
            entity.velocity = new_velocity;
            entity.position = position;
        }).await;
    }
}
