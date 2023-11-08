use super::*;

#[derive(Default)]
#[inherit(Entity)]
pub struct FallingBlock {
    pub entity: Entity,
    pub spawn_position: BlockPosition,
}
