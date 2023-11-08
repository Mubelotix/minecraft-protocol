use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractSkeleton {
    pub monster: Monster,
}

#[derive(Default)]
#[inherit(AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Skeleton {
    pub abstract_skeleton: AbstractSkeleton,
}
