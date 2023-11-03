use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Axolot {
    pub animal: Animal,
    pub variant: u8,
    pub playing_dead: bool,
    pub spawn_from_bucket: bool,
}
