use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Entity)]
pub struct ItemFrame {
    pub entity: Entity,
    pub item: Slot,
    pub rotation: u8,
}
