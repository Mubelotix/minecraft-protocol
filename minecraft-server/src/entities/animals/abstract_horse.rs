use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractHorse {
    pub animal: Animal,
    pub mask: u8,
}

