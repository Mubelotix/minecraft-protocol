use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { ElderGuardian },
)]
pub struct Guardian {
    pub monster: Monster,
    pub is_retracting_spikes: bool,
    pub target_eid: Eid,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Guardian, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct ElderGuardian {
    pub guardian: Guardian,
}

