use super::*;

#[derive(Default)]
#[inherit(AbstractGolem, PathfinderMob, Mob, LivingEntity, Entity)]
pub struct Shulker {
    pub abstract_golem: AbstractGolem,
    pub attach_face: u8,
    pub attach_position: Option<BlockPosition>,
    pub shield_height: u8,
    pub color: u8,
}
