use super::*;

#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct Interaction {
    pub entity: Entity,
    pub width: f32,
    pub height: f32,
    pub responsive: bool,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction {
            entity: Entity::default(),
            width: 1.0,
            height: 1.0,
            responsive: false,
        }
    }
}
