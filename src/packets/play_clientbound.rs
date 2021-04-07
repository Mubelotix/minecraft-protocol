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

/// This packet is used for a number of actions and animations performed by blocks, usually non-persistent.
/// 
/// See [Block Actions](https://wiki.vg/Block_Actions) for a list of values.
/// 
/// **Warning**: This packet uses a block ID, not a block state.
#[derive(Debug, MinecraftPacket)]
pub struct BlockAction {
    /// Block coordinates
    pub location: Position,
    /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
    pub action_id: u8,
    /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
    pub action_param: u8,
    /// The block type ID for the block. This must match the block at the given coordinates
    pub block_type: VarInt,
}

/// Fired whenever a block is changed within the render distance.
/// Changes include plant growth, cake bites, redstone repeater delay changes, block facing changes (bed, chest, hopper...) and many other values depending on the type of the block.
/// 
/// **Warning**: Changing a block in a chunk that is not loaded is not a stable action. The Notchian client currently uses a shared empty chunk which is modified for all block changes in unloaded chunks; while in 1.9 this chunk never renders in older versions the changed block will appear in all copies of the empty chunk. Servers should avoid sending block changes in unloaded chunks and clients should ignore such packets.
#[derive(Debug, MinecraftPacket)]
pub struct BlockChange {
    /// Block Coordinates
    pub location: Position,
    /// The new block state ID for the block as given in the [global palette](http://minecraft.gamepedia.com/Data_values%23Block_IDs). See that section for more information.
    pub block_state: VarInt,
}

#[derive(Debug, MinecraftPacket)]
pub struct BossBar<'a> {
    /// Unique ID for this bar.
    pub uuid: UUID,
    /// The action to apply on the boss bar.
    pub action: crate::boss_bar::BossBarAction<'a>,
}

/// Changes the difficulty setting in the client's option menu
#[derive(Debug, MinecraftPacket)]
pub struct ServerDifficulty {
    pub difficulty: crate::difficulty::Difficulty,
    pub difficulty_locked: bool,
}

/// Identifying the difference between Chat/System Message is important as it helps respect the user's chat visibility options. See [processing chat](https://wiki.vg/Chat#Processing_chat) for more info about these positions.
/// 
/// **Warning**: Game info accepts json formatting but does not display it, although the deprecated §-based formatting works. This is not an issue when using the [Title] packet, so prefer that packet for displaying information in that slot. See MC-119145 for more information.
#[derive(Debug, MinecraftPacket)]
pub struct ChatMessage<'a> {
    pub message: Chat<'a>,
    pub position: crate::chat::Position,
    /// Used by the Notchian client for the disableChat launch option. Setting 0 will always display the message regardless of the setting.
    pub sender: UUID,
}
