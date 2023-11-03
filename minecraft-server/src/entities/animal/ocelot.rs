use minecraft_protocol::packets::UUID;
use super::*;

#[derive(Default)]
#[inherit(Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Ocelot {
    pub animal: Animal,
    pub is_trusting: bool,
}
