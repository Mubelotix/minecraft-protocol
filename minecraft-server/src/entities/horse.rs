use super::*;

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Horse {
    pub abstract_horse: AbstractHorse,
    pub variant: usize,
}

