use super::*;

#[derive(Default)]
#[MinecraftEntity(
    inheritable,
    ancestors { Entity },
    descendants { GlowingItemFrame },
)]
pub struct ItemFrame {
    pub entity: Entity,
    pub item: Slot,
    pub rotation: u8,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { ItemFrame, Entity },
)]
pub struct GlowingItemFrame {
    pub item_frame: ItemFrame,
}

#[derive(Default)]
#[MinecraftEntity(
    ancestors { Entity },
)]
pub struct ItemEntity {
    pub entity: Entity,
    pub item: Slot,
}
