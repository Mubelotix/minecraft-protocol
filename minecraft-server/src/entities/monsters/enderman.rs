use minecraft_protocol::ids::blocks::Block;

use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Monster, PathfinderMob, Mob, LivingEntity, Entity },
)]
pub struct Enderman {
    pub monster: Monster,
    pub block_id: Option<Block>,
    pub is_screaming: bool,
    pub is_staring: bool,
}
