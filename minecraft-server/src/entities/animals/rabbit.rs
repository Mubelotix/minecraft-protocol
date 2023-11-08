use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Rabbit {
    pub animal: Animal,
    pub variant: u16,
}
