use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Giant {
    pub monster: Monster,
}
