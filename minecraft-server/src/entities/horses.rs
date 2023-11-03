use super::*;

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Horse {
    pub abstract_horse: AbstractHorse,
    pub variant: usize,
}

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ZombieHorse {
    pub abstract_horse: AbstractHorse,
}

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct SkeletonHorse {
    pub abstract_horse: AbstractHorse,
}
