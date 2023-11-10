use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Bee {
    pub animal: Animal,
    pub flag: u8,
    pub anger_time: usize,
}
