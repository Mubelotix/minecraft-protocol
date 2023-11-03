use super::*;

#[inherit(Entity)]
pub struct AbstractArrow {
    pub entity: Entity,
    pub is_critical: bool,
    pub is_no_clip: bool,
    pub piercing_level: i32,
}

