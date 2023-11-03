use minecraft_protocol::packets::UUID;

use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct TameableAnimal {
    pub animal: Animal,
    pub action_mask: u8,
    pub owner: Option<UUID>,
}
