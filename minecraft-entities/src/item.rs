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

impl TryAsEntityRef<ItemFrame> for AnyEntity {
    fn try_as_entity_ref(&self) -> Option<&ItemFrame> {
        match self {
            AnyEntity::ItemFrame(item_frame) => Some(&item_frame),
            AnyEntity::GlowingItemFrame(glowing_item_frame) => Some(&glowing_item_frame.item_frame),
            _ => None,
        }
    }

    fn try_as_entity_mut(&mut self) -> Option<&mut ItemFrame> {
        match self {
            AnyEntity::ItemFrame(item_frame) => Some(item_frame),
            AnyEntity::GlowingItemFrame(glowing_item_frame) => Some(&mut glowing_item_frame.item_frame),
            _ => None,
        }
    }
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
