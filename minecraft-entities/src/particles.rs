use minecraft_protocol::components::particle::Particle;

use super::*;

#[inherit(Entity)]
pub struct AreaEffectCloud {
    pub entity: Entity,
    pub radius: f32,
    pub color: Option<usize>,
    pub ignore_radius: bool,
    pub particle: Particle,
}

impl Default for AreaEffectCloud {
    fn default() -> Self {
        Self {
            entity: Entity::default(),
            radius: 0.5,
            color: None,
            ignore_radius: false,
            particle: Particle::Effect,
        }
    }
}

#[derive(Default)]
#[inherit(Entity)]
pub struct FishingHook{
    pub entity: Entity,
    pub hooked_entity: Option<usize>,
    pub is_catchable: bool,
}
