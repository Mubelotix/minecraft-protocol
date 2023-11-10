use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Axolotl {
    pub animal: Animal,
    pub variant: u8,
    pub playing_dead: bool,
    pub spawn_from_bucket: bool,
}
