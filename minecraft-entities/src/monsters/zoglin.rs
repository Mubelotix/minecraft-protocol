use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Zoglin {
    pub monster: Monster,
    pub is_baby: bool,
}
