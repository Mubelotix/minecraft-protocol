use super::*;

#[derive(Default)]
#[inherit(AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct ZombieHorse {
    pub abstract_horse: AbstractHorse,
}

