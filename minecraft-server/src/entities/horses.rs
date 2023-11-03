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

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Camel {
    pub abstract_horse: AbstractHorse,
    pub is_dashing: bool,
    pub last_pose_change_tick: usize,
}

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ChestedHorse {
    pub abstract_horse: AbstractHorse,
    pub has_chest: bool,
}
