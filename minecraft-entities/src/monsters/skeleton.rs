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

#[derive(Default)]
#[inherit(AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct WitherSkeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[inherit(AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Stray {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[inherit(Entity)]
pub struct WitherSkull {
    pub entity: Entity,
    pub is_invulnerable: bool,
}
