use super::*;

#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Wither {
    pub monster: Monster,
    pub center_head_target: Option<Eid>,
    pub left_head_target: Option<Eid>,
    pub right_head: Option<Eid>,
    pub invulnerable_time: usize,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct WitherSkull {
    pub entity: Entity,
    pub is_invulnerable: bool,
}
