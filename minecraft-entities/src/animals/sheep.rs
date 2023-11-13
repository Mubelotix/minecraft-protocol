use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Sheep {
    pub animal: Animal,
    pub mask_style: u8,
}
