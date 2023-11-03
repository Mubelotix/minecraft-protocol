use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Entity)]
pub struct AbstractArrow {
    pub entity: Entity,
    pub is_critical: bool,
    pub is_no_clip: bool,
    pub piercing_level: i32,
}

#[inherit(AbstractArrow, Entity)]
pub struct Arrow {
    pub abstract_arrow: AbstractArrow,
    pub color: isize,
}

impl Default for Arrow {
    fn default() -> Self {
        Self {
            abstract_arrow: AbstractArrow::default(),
            color: -1,
        }
    }
}
