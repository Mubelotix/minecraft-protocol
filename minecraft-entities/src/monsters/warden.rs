use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Warden {
    pub monster: Monster,
    pub anger_level: usize,
}

