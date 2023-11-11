use super::*;

#[MinecraftEntity(
    ancestors { TameableAnimal, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Wolf {
    pub tameable_animal: TameableAnimal,
    pub is_begging: bool,
    pub collar_color: u8,
    pub anger: u16,
}

impl Default for Wolf {
    fn default() -> Self {
        Self {
            tameable_animal: Default::default(),
            is_begging: false,
            collar_color: 14,
            anger: 0,
        }
    }
}
