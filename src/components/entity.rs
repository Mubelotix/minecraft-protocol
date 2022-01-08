use std::collections::BTreeMap;

use crate::{nbt::NbtTag, *};

#[derive(Debug, MinecraftPacketPart)]
pub struct EntityAttribute<'a> {
    pub value: f64,
    pub modifiers: Array<'a, EntityAttributeModifier, VarInt>,
}

/// To make the sum of modifiers, apply all modifiers with `operation` [EntityAttributeModifierOperation::Add], then all with [EntityAttributeModifierOperation::AddProportion], and finally all with [EntityAttributeModifierOperation::Multiply].
#[derive(Debug, MinecraftPacketPart)]
pub struct EntityAttributeModifier {
    pub uuid: UUID,
    /// May be positive or negative
    pub amount: f64,
    /// The way the modifier must be applied
    pub operation: EntityAttributeModifierOperation,
}

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum EntityAttributeModifierOperation {
    /// `value = base_value + modifier`
    Add,
    /// `value = base_value + base_value * modifier`
    AddProportion,
    /// `value = base_value * modifier`
    Multiply,
}

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum EntityInteraction {
    Interact {
        hand: super::slots::Hand,
    },
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
        hand: super::slots::Hand,
    },
}

#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerAction {
    StartSneaking,
    StopSneaking,
    /// Leave bed is only sent when the “Leave Bed” button is clicked on the sleep GUI, not when waking up due today time.
    LeaveBed,
    StartSprinting,
    StopSprinting,
    StartJumpWithHorse,
    StopJumpWithHorse,
    /// Open horse inventory is only sent when pressing the inventory key (default: E) while on a horse — all other methods of opening a horse's inventory (involving right-clicking or shift-right-clicking it) do not use this packet.
    OpenHorseInventory,
    StartFlyingWithElytra,
}

#[minecraft_enum(VarInt)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pose {
    Standing,
    FallFlying,
    Sleeping,
    Swimming,
    SpinAttack,
    Sneaking,
    Dying,
    LongJumping,
}

#[derive(Debug, Clone)]
pub struct EntityMetadata {
    pub items: BTreeMap<u8, EntityMetadataValue>,
}

impl<'a> MinecraftPacketPart<'a> for EntityMetadata {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        for (key, value) in self.items.into_iter() {
            key.serialize_minecraft_packet_part(output)?;
            value.serialize_minecraft_packet_part(output)?;
        }
        0xff.serialize_minecraft_packet_part(output)?;
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        mut input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let mut items = BTreeMap::new();
        loop {
            let (key, new_input) = u8::deserialize_minecraft_packet_part(input)?;
            if key == 0xff {
                input = new_input;
                break;
            }
            let (value, new_input) =
                EntityMetadataValue::deserialize_minecraft_packet_part(new_input)?;
            input = new_input;
            items.insert(key, value);
        }

        Ok((EntityMetadata { items }, input))
    }
}

#[derive(Debug, Clone, MinecraftPacketPart)]
#[discriminant(u8)]
pub enum EntityMetadataValue {
    Byte {
        value: u8,
    },
    VarInt {
        value: VarInt,
    },
    Float {
        value: f32,
    },
    String {
        value: String,
    },
    Chat {
        chat: String,
    },
    OptionChat {
        chat: Option<String>,
    },
    Slot {
        slot: super::slots::Slot,
    },
    Bool {
        value: bool,
    },
    Rotation {
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
    },
    Position {
        position: Position,
    },
    OptionPosition {
        position: Option<Position>,
    },
    Direction {
        direction: Direction,
    },
    OptionUUID {
        uuid: Option<UUID>,
    },
    /// Use [Block::from_state_id](crate::ids::blocks::Block::from_state_id) to get the block.
    OptionBlockStateID {
        block_state_id: VarInt,
    },
    Nbt {
        value: NbtTag,
    },
    Particle {
        particle: super::particle::Particle,
    },
    Villager {
        villager_type: super::trades::VillagerType,
        profession: super::trades::VillagerProfession,
        level: super::trades::VillagerLevel,
    },
    OptionVarInt {
        /// 0 for absent; 1 + actual value otherwise. Used for entity IDs.
        option_varint: VarInt,
    },
    Pose {
        pose: Pose,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_metadata() {
        let t1 = [
            /*68, 160, 129, 2, */ 0, 0, 0, 2, 5, 0, 6, 18, 0, 4, 7, 0, 15, 13, 10, 14, 0, 0,
            12, 1, 0, 13, 10, 0, 8, 2, 66, 32, 0, 0, 9, 1, 0, 11, 1, 0, 10, 7, 0, 1, 1, 172, 2, 3,
            7, 0, 7, 0, 0, 5, 7, 0, 16, 7, 0, 17, 7, 0, 255,
        ];
        let t2 = [/*68, 219, 242, 1, */ 15, 13, 68, 255];

        EntityMetadata::deserialize_uncompressed_minecraft_packet(&t1).unwrap();
        EntityMetadata::deserialize_uncompressed_minecraft_packet(&t2).unwrap();
    }
}
