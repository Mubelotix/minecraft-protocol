use super::*;

#[derive(Debug, MinecraftPacket)]
pub struct SpawnEntity {
    pub id: VarInt,
    pub uuid: UUID,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: Angle,
    pub yaw: Angle,
    pub data: i32,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnLivingEntity {
    pub id: VarInt,
    pub uuid: UUID,
    pub entity_type: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Angle,
    pub pitch: Angle,
    pub head_pitch: Angle,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnPainting {
    pub id: VarInt,
    pub uuid: UUID,
    pub motive: crate::paintings::Painting,
    pub location: Position,
    pub direction: Direction,
}

#[derive(Debug, MinecraftPacket)]
pub struct SpawnPlayer {
    pub id: VarInt,
    pub uuid: UUID,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Angle,
    pub pitch: Angle,
}

#[derive(Debug, MinecraftPacket)]
pub struct EntityAnimation {
    pub id: VarInt,
    pub animation: crate::animations::Animation,
}

#[derive(Debug, MinecraftPacket)]
pub struct Statistics<'a> {
    pub count: VarInt,
    pub statistic: RawBytes<'a>,
}

#[derive(Debug, MinecraftPacket)]
pub struct AcknowledgePlayerDigging {
    pub location: Position,
    pub block: VarInt,
    pub status: crate::blocks::PartialDiggingState,
    pub successful: bool,
}

/// 0–9 are the displayable destroy stages and each other number means that there is no animation on this coordinate.
/// 
/// Block break animations can still be applied on air; the animation will remain visible although there is no block being broken. However, if this is applied to a transparent block, odd graphical effects may happen, including water losing its transparency. (An effect similar to this can be seen in normal gameplay when breaking ice blocks)
/// 
/// If you need to display several break animations at the same time you have to give each of them a unique Entity ID. The entity ID does not need to correspond to an actual entity on the client. It is valid to use a randomly generated number.
#[derive(Debug, MinecraftPacket)]
pub struct BlockBreakAnimation {
    /// Entity ID of the entity breaking the block
    pub id: VarInt,
    /// Block Position
    pub location: Position,
    /// 0–9 to set it, any other value to remove it
    pub destroy_stage: u8,
}

/// Sets the block entity associated with the block at the given location.
#[derive(Debug, MinecraftPacket)]
pub struct BlockEntityData<'a> {
    pub location: Position,
    /// The type of update to perform, see [crate::blocks::BlockEntityDataAction].
    pub action: crate::blocks::BlockEntityDataAction,
    /// Data to set. May be [crate::nbt::NbtTag::Null], in which case the block entity at the given location is removed (though this is not required since the client will remove the block entity automatically on chunk unload or block removal).
    pub data: crate::nbt::NbtTag<'a>,
}
