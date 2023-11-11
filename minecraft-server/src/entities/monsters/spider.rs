use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Spider {
    pub monster: Monster,
    pub is_climbing_mask: u8,
}
