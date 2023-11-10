use super::*;

#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct FallingBlock {
    pub entity: Entity,
    pub spawn_position: BlockPosition,
}
