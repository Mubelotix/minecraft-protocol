use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct BasePiglin {
    pub monster: Monster,
    pub is_immune: bool,
}
