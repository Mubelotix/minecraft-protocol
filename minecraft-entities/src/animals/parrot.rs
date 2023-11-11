use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { TameableAnimal, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Parrot {
    pub tameable_animal: TameableAnimal,
    pub variant: u8,
}

