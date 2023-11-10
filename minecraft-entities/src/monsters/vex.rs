use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Vex {
    pub monster: Monster,
}
