use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Animal {
    pub ageable_mob: AgeableMob,
}
