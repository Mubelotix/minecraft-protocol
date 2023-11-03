use super::*;

#[derive(Default)]
#[inherit(TameableAnimal, Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Cat {
    pub tameable_animal: TameableAnimal,
    pub variant: u8,
    pub is_lying: bool,
    pub is_relaxed: bool,
    pub collar_color: u8,
}

impl Default for Cat {
    fn default() -> Self {
        Self {
            tameable_animal: Default::default(),
            variant: 0,
            is_lying: false,
            is_relaxed: false,
            collar_color: 14,
        }
    }
}
