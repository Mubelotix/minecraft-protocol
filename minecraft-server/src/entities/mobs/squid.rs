use super::*;

#[derive(Default)]
#[inherit(WaterAnimal, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Squid {
    pub water_animal: WaterAnimal,
}
