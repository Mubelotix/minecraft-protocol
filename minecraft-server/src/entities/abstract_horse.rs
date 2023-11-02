use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct AbstractHorse {
    pub mask: u8,
}

