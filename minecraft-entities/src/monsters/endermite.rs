use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Endermite {
    pub monster: Monster,
}
