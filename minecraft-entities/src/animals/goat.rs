use super::*;

#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Goat {
    pub animal: Animal,
    pub is_screaming: bool,
    pub has_left_horn: bool,
    pub has_right_horn: bool,
}

impl Default for Goat {
    fn default() -> Self {
        Self {
            animal: Animal::default(),
            is_screaming: false,
            has_left_horn: true,
            has_right_horn: true,
        }
    }
}
