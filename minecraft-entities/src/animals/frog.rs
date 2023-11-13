use minecraft_protocol::packets::UUID;
use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Frog {
    pub animal: Animal,
    pub variant: u8,
    pub tongue_target: Option<usize>,
}
