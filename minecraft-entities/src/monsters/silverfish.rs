use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Silverfish {
    pub monster: Monster,
}
