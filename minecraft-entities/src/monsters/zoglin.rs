use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Zoglin {
    pub monster: Monster,
    pub is_baby: bool,
}
