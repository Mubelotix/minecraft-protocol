use super::*;

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct FallingBlock {
    pub entity: Entity,
    pub spawn_position: BlockPosition,
}
