use super::*;

#[derive(Default)]
#[inheritable]
#[inherit(Entity)]
pub struct ItemFrame {
    pub entity: Entity,
    pub item: Slot,
    pub rotation: u8,
}

#[derive(Default)]
#[inherit(ItemFrame, Entity)]
pub struct GlowingItemFrame {
    pub item_frame: ItemFrame,
}
