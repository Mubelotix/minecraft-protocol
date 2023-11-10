use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct PolarBear {
    pub animal: Animal,
    pub is_standing: bool,
}
