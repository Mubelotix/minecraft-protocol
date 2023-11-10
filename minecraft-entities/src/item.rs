use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable, parents { Entity },
)]
pub struct ItemFrame {
    pub entity: Entity,
    pub item: Slot,
    pub rotation: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { ItemFrame, Entity },
)]
pub struct GlowingItemFrame {
    pub item_frame: ItemFrame,
}

#[derive(Default)]
#[MinecraftEntity(
    parents { Entity },
)]
pub struct ItemEntity {
    pub entity: Entity,
    pub item: Slot,
}
