use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Guardian {
    pub monster: Monster,
    pub is_retracting_spikes: bool,
    pub target_eid: Eid,
}

#[derive(Default)]
#[inheritable]
#[inherit(Guardian, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ElderGuardian {
    pub guardian: Guardian,
}

