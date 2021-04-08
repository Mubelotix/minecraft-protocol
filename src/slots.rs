use crate::{*, nbt::NbtTag};

/// The [Slot] data structure is how Minecraft represents an item and its associated data in the [Minecraft Protocol](https://wiki.vg/Protocol).
#[derive(Debug, MinecraftPacketPart)]
pub struct Slot<'a> {
    /// The [item ID](http://minecraft.gamepedia.com/Java_Edition_data_values%23Blocks).
    /// Omitted if present is false.
    /// Item IDs are distinct from block IDs; see [Data Generators](https://wiki.vg/Data_Generators) for more information
    pub item_id: VarInt,
    pub item_count: VarInt,
    /// Things like enchantements and durability are encoded in this field.
    pub nbt_data: NbtTag<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slot() {
        let serialized = &mut [0x01, 0x01, 0x01, 0x00];
        let deserialized = <Option<Slot>>::deserialize_minecraft_packet(serialized).unwrap().unwrap();
        assert_eq!(deserialized.item_id.0, 1);
        assert_eq!(deserialized.item_count.0, 1);
        assert!(matches!(deserialized.nbt_data, NbtTag::Null));
    }
}