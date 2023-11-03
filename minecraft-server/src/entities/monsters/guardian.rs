use super::*;

#[derive(Default)]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Guardian {
    pub monster: Monster,
    pub is_retracting_spikes: bool,
    pub target_eid: Eid,
}
