use super::*;

#[derive(Default)]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Giant {
    pub monster: Monster,
}
