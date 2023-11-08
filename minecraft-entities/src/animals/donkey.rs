use super::*;

#[derive(Default)]
#[inherit(ChestedHorse, AbstractHorse, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Donkey {
    pub chested_horse: ChestedHorse,
}
