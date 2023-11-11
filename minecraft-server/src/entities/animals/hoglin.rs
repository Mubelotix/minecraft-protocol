use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Hoglin {
    pub animal: Animal,
    pub is_immune: bool,
}
