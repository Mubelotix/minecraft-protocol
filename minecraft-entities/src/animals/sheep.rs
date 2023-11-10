use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Sheep {
    pub animal: Animal,
    pub mask_style: u8,
}
