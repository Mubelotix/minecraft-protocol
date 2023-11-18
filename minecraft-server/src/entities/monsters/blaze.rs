use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Blaze {
    pub monster: Monster,
    pub is_on_fire: bool,
}
