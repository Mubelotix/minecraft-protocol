use crate::*;

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
        hand: crate::slots::Hand,
    },
    Attack,
    InteractAt {
        target_x: f32,
        target_y: f32,
        target_z: f32,
        hand: crate::slots::Hand,
    },
}

#[minecraft_enum(VarInt)]
#[derive(Debug)]
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
