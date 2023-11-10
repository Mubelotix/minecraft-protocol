use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct AbstractSkeleton {
    pub monster: Monster,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Skeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WitherSkeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Stray {
    pub abstract_skeleton: AbstractSkeleton,
}
