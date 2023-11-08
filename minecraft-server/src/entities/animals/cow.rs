use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Cow {
    pub animal: Animal,
}

#[derive(Default)]
#[inherit(Cow, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]

pub struct Mooshroom {
    pub cow: Cow,
    pub variant: u8, // In the doc it is a string 
}
