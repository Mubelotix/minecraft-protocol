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
