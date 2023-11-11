use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Ocelot {
    pub animal: Animal,
    pub is_trusting: bool,
}
