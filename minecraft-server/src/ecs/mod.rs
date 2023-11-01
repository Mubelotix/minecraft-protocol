pub mod entities;
pub mod tags;
pub mod components;

pub use entities::*;
pub use tags::*;
pub use components::*;

/*
struct HealthComponent {
    health: i32,
    max_health: i32,
}

struct PositionComponent {
    x: i32,
    y: i32,
    z: i32,
}

enum EntityType {
    Player,
    Enemy,
    Item,
}

struct Entity {
    id: i32,
    ty: EntityType,
}

struct World {
}

impl World {
    async fn spawn_player(&self) -> i32 {
        let id = self.entities.len() as i32;
        self.entities.insert(id, Entity { id, ty: EntityType::Player });
        self.health_components.insert(id, HealthComponent { health: 100, max_health: 100 });
        self.position_components.insert(id, PositionComponent { x: 0, y: 0, z: 0 });
        id
    }

    async fn move(&self, entity: i32, x: i32, y: i32, z: i32) {
        if let Some(position) = self.position_components.get_mut(&entity) {
            let cx = position.x / 16;
            let cy = position.y / 16;
            self.chunks.entry((cx, cy)).or_insert_with(|| BTreeSet::new()).remove(&entity);
            
            position.x += x;
            position.y += y;
            position.z += z;
            self.chunks.entry((cx, cy)).or_insert_with(|| BTreeSet::new()).insert(entity);
        }
    }

    async fn inflict_damage(&self, id: i32, damage: i32) {
        if let Some(health) = self.health_components.get_mut(&id) {
            health.health -= damage;
        }
    }
}
*/