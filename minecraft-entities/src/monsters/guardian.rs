use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Guardian {
    pub monster: Monster,
    pub is_retracting_spikes: bool,
    pub target_eid: Eid,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Guardian, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ElderGuardian {
    pub guardian: Guardian,
}

