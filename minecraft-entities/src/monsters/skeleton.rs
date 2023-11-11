use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
    descendants { Skeleton, WitherSkeleton, Stray },
)]
pub struct AbstractSkeleton {
    pub monster: Monster,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Skeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct WitherSkeleton {
    pub abstract_skeleton: AbstractSkeleton,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { AbstractSkeleton, Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Stray {
    pub abstract_skeleton: AbstractSkeleton,
}
