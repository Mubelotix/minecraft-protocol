use minecraft_protocol::packets::UUID;
use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Fox {
    pub animal: Animal,
    pub variant: u8,
    pub mask: u8,
    pub first_uuid: Option<UUID>,
    pub second_uuid: Option<UUID>,
}
