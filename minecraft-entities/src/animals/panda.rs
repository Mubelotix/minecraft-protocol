use minecraft_protocol::packets::UUID;
use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Animal, AgeableMob, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Panda {
    pub animal: Animal,
    pub breed_timer: u16,
    pub sneeze_timer: u16,
    pub eat_timer: u16,
    pub main_gene: u8,
    pub hidden_gene: u8,
    pub action_mask: u8,
}
