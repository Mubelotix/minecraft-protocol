use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct PolarBear {
    pub animal: Animal,
    pub is_standing: bool,
}
