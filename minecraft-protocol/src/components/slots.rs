use crate::{nbt::NbtTag, *};

/// The [Slot] data structure is how Minecraft represents an item and its associated data in the [Minecraft Protocol](https://wiki.vg/Protocol).
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, MinecraftPacketPart)]
pub struct Slot {
    /// `Some(item)` if there is an item in this slot; `None` if it is empty.
    pub item: Option<SlotItem>,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone, MinecraftPacketPart)]
pub struct SlotItem {
    /// The [item](crate::ids::items::Item).
    /// Item IDs are distinct from [block IDs](crate::ids::blocks::Block); see [crate::ids] for more information.
    pub item_id: crate::ids::items::Item,
    pub item_count: i8,
    /// Things like enchantements and durability are encoded in this field.
    pub nbt_data: NbtTag,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum Hand {
    MainHand,
    OffHand,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum MainHand {
    Left,
    Right,
}

#[minecraft_enum(u8)]
#[derive(Debug, Clone, Copy)]
pub enum EquipmentSlot {
    MainHand,
    OffHand,
    Boots,
    Leggings,
    Chestplate,
    Helmet,
}

impl std::cmp::PartialEq for EquipmentSlot {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8).eq(&(*other as u8))
    }
}
impl std::cmp::Eq for EquipmentSlot {}
impl std::cmp::PartialOrd for EquipmentSlot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for EquipmentSlot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

use std::collections::BTreeMap;
#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct EquipmentSlotArray {
    pub slots: BTreeMap<EquipmentSlot, Slot>,
}


impl<'a> MinecraftPacketPart<'a> for EquipmentSlotArray {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len = self.slots.len();
        for (idx, (slot_index, slot)) in self.slots.into_iter().enumerate() {
            let mut slot_index = slot_index as u8;
            if idx + 1 < len {
                slot_index += 0b1000_0000;
            }
            output.push(slot_index);
            slot.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        mut input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let mut slots = BTreeMap::new();
        loop {
            let (number, new_input) = u8::deserialize_minecraft_packet_part(input)?;
            let (slot, new_input) = Slot::deserialize_minecraft_packet_part(new_input)?;
            input = new_input;

            let slot_index = 0b0111_1111 & number;
            let slot_index_variant: EquipmentSlot = if slot_index <= 5 {
                unsafe { std::mem::transmute(slot_index) }
            } else {
                return Err("The slot index cannot be higher than 5.");
            };
            slots.insert(slot_index_variant, slot);

            if number < 0b1000_0000 {
                break;
            }
        }
        Ok((EquipmentSlotArray { slots }, input))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum WindowType {
    OneRow,
    TwoRows,
    ThreeRows,
    FourRows,
    FiveRows,
    SixRows,
    ThreeByThree,
    Anvil,
    Beacon,
    BlastFurnace,
    BrewingStand,
    Crafting,
    Enchantment,
    Furnace,
    Grindstone,
    Hopper,
    Lectern,
    Loom,
    Merchant,
    ShulkerBox,
    Smithing,
    Smoker,
    Cartography,
    Stonecutter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot() {
        let serialized = &mut [0x01, 0x01, 0x01, 0x00];
        let deserialized = Slot::deserialize_uncompressed_minecraft_packet(serialized)
            .unwrap()
            .item
            .unwrap();
        assert_eq!(deserialized.item_id, crate::ids::items::Item::Stone);
        assert_eq!(deserialized.item_count, 1);
        assert!(matches!(deserialized.nbt_data, NbtTag::Null));
    }
}
