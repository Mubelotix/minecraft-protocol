use super::*;

#[derive(Debug, MinecraftPacketPart)]
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

#[derive(Debug, MinecraftPacketPart)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[derive(Debug, MinecraftPacketPart)]
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

#[derive(Debug, MinecraftPacketPart)]
pub struct SpawnPainting {
    pub id: VarInt,
    pub uuid: UUID,
    pub motive: crate::paintings::Painting,
    pub location: Position,
    pub direction: Direction,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct SpawnPlayer {
    pub id: VarInt,
    pub uuid: UUID,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: Angle,
    pub pitch: Angle,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct EntityAnimation {
    pub id: VarInt,
    pub animation: crate::animations::Animation,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct Statistics<'a> {
    pub count: VarInt,
    pub statistic: RawBytes<'a>,
}

#[derive(Debug, MinecraftPacketPart)]
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
#[derive(Debug, MinecraftPacketPart)]
pub struct BlockBreakAnimation {
    /// Entity ID of the entity breaking the block
    pub id: VarInt,
    /// Block Position
    pub location: Position,
    /// 0–9 to set it, any other value to remove it
    pub destroy_stage: u8,
}

/// Sets the block entity associated with the block at the given location.
#[derive(Debug, MinecraftPacketPart)]
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
#[derive(Debug, MinecraftPacketPart)]
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
#[derive(Debug, MinecraftPacketPart)]
pub struct BlockChange {
    /// Block Coordinates
    pub location: Position,
    /// The new block state ID for the block as given in the [global palette](http://minecraft.gamepedia.com/Data_values%23Block_IDs). See that section for more information.
    pub block_state: VarInt,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct BossBar<'a> {
    /// Unique ID for this bar.
    pub uuid: UUID,
    /// The action to apply on the boss bar.
    pub action: crate::boss_bar::BossBarAction<'a>,
}

/// Changes the difficulty setting in the client's option menu
#[derive(Debug, MinecraftPacketPart)]
pub struct ServerDifficulty {
    pub difficulty: crate::difficulty::Difficulty,
    pub difficulty_locked: bool,
}

/// Identifying the difference between Chat/System Message is important as it helps respect the user's chat visibility options. See [processing chat](https://wiki.vg/Chat#Processing_chat) for more info about these positions.
/// 
/// **Warning**: Game info accepts json formatting but does not display it, although the deprecated §-based formatting works. This is not an issue when using the [Title] packet, so prefer that packet for displaying information in that slot. See MC-119145 for more information.
#[derive(Debug, MinecraftPacketPart)]
pub struct ChatMessage<'a> {
    pub message: Chat<'a>,
    pub position: crate::chat::Position,
    /// Used by the Notchian client for the disableChat launch option. Setting 0 will always display the message regardless of the setting.
    pub sender: UUID,
}

/// The server responds with a list of auto-completions of the last word sent to it.
/// In the case of regular chat, this is a player username.
/// Command names and parameters are also supported.
/// The client sorts these alphabetically before listing them.
#[derive(Debug, MinecraftPacketPart)]
pub struct TabComplete<'a> {
    /// Transaction ID
    pub id: VarInt,
    /// Start of the text to replace
    pub start: VarInt,
    /// Length of the text to replace
    pub lenght: VarInt,
    /// Eligible values to insert, note that each command is sent separately instead of in a single string, hence the need for an [Array].
    pub matches: Array<'a, crate::auto_completion::Match<'a>, VarInt>,
}

/// Lists all of the commands on the server, and how they are parsed.
/// This is a directed graph, with one root node. Each redirect or child node must refer only to nodes that have already been declared.
#[derive(Debug, MinecraftPacketPart)]
pub struct DeclareCommands<'a> {
    pub count: VarInt,
    /// An array of [Node](https://wiki.vg/Command_Data) followed by the index of the `root` node in the array.
    /// Parsing is unimplemented yet.
    pub data: RawBytes<'a>,
}

/// A packet from the server indicating whether a request from the client was accepted, or whether there was a conflict (due to lag).
/// If the packet was not accepted, the client must respond with a serverbound window confirmation packet.
#[derive(Debug, MinecraftPacketPart)]
pub struct WindowConfirmation {
    /// The ID of the window that the action occurred in.
    pub window_id: u8,
    /// Every action that is to be accepted has a unique ID. This number is an incrementing integer (starting at 0) with separate counts for each window ID.
    pub action_id: i16,
    /// Whether the action was accepted.
    pub accepted: bool,
}

/// This packet is sent from the server to the client when a window is forcibly closed, such as when a chest is destroyed while it's open.
#[derive(Debug, MinecraftPacketPart)]
pub struct CloseWindow {
    /// This is the ID of the window that was closed. 0 for inventory.
    pub window_id: u8,
}

/// Sent by the server when items in multiple slots (in a window) are added/removed.
/// This includes the main inventory, equipped armour and crafting slots.
#[derive(Debug, MinecraftPacketPart)]
pub struct WindowItems<'a> {
    /// The ID of window which items are being sent for. 0 for player inventory.
    pub window_id: u8,
    /// The [crate::slots::Slot]s in this window.
    /// See [inventory windows](https://wiki.vg/Inventory#Windows) for further information about how slots are indexed.
    pub slots: Array<'a, Option<crate::slots::Slot<'a>>, i16>,
}

/// This packet is used to inform the client that part of a GUI window should be updated.
#[derive(Debug, MinecraftPacketPart)]
pub struct WindowProperty {
    pub window_id: u8,
    /// The property to be updated.
    /// The meaning of this field depends on the type of the window.
    /// The [the wiki](https://wiki.vg/Protocol#Window_Property) shows the known combinations of window type and property, and how the value is to be interpreted.
    pub property: i16,
    /// The new value for the property.
    /// The meaning of this field depends on the type of the window.
    /// The [the wiki](https://wiki.vg/Protocol#Window_Property) shows the known combinations of window type and property, and how the value is to be interpreted.
    pub value: i16,
}

/// Sent by the server when an item in a slot (in a window) is added/removed.
/// 
/// To set the cursor (the item currently dragged with the mouse), use -1 as `window_id` and as `slot_index`.
/// 
/// This packet can only be used to edit the hotbar of the player's inventory if window ID is set to 0 (slots 36 through 44). If the window ID is set to -2, then any slot in the inventory can be used but no add item animation will be played.
#[derive(Debug, MinecraftPacketPart)]
pub struct SetSlot<'a> {
    /// The window which is being updated. 0 for player inventory.
    /// Note that all known window types include the player inventory.
    /// This packet will only be sent for the currently opened window while the player is performing actions, even if it affects the player inventory.
    /// After the window is closed, a number of these packets are sent to update the player's inventory window (0).
    pub window_id: i8,
    /// The slot that should be updated.
    pub slot_index: i16,
    pub slot_value: crate::slots::Slot<'a>,
}

/// Applies a cooldown period to all items with the given type.
/// Used by the Notchian server with enderpearls.
/// This packet should be sent when the cooldown starts and also when the cooldown ends (to compensate for lag), although the client will end the cooldown automatically.
/// Can be applied to any item, note that interactions still get sent to the server with the item but the client does not play the animation nor attempt to predict results (i.e block placing).
#[derive(Debug, MinecraftPacketPart)]
pub struct SetCooldown {
    /// Numeric ID of the item to apply a cooldown to.
    pub item_id: VarInt,
    /// Number of ticks to apply a cooldown for, or 0 to clear the cooldown.
    pub cooldown_ticks: VarInt,
}

/// Mods and plugins can use this to send their data.
/// Minecraft itself uses several [plugin channels](https://wiki.vg/Plugin_channel).
/// These internal channels are in the `minecraft` namespace.
/// 
/// [More documentation](http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)
#[derive(Debug, MinecraftPacketPart)]
pub struct PluginMessage<'a> {
    /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
    pub identifier: Identifier<'a>,
    /// Any data, depending on the channel.
    /// `minecraft:` channels are documented [here](https://wiki.vg/Plugin_channel).
    /// The length of this array must be inferred from the packet length.
    pub data: RawBytes<'a>,
}

/// Used to play a sound effect on the client.
/// Custom sounds may be added by resource packs.
#[derive(Debug, MinecraftPacketPart)]
pub struct NamedSoundEffect<'a> {
    /// All sound effect names as of 1.16.5 can be seen [here](https://pokechu22.github.io/Burger/1.16.5.html#sounds).
    pub sound_name: Identifier<'a>,
    /// The category that this sound will be played from ([current categories](https://gist.github.com/konwboj/7c0c380d3923443e9d55)).
    pub sound_category: VarInt,
    /// Effect X multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
    pub effect_position_x: i32,
    /// Effect Y multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
    pub effect_position_y: i32,
    /// Effect Z multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
    pub effect_position_z: i32,
    /// `1.0` is 100%, can be more.
    pub volume: f32,
    /// Float between 0.5 and 2.0 by Notchian clients.
    pub pitch: f32,
}

/// Sent by the server before it disconnects a client.
/// The client assumes that the server has already closed the connection by the time the packet arrives.
#[derive(Debug, MinecraftPacketPart)]
pub struct Disconnect<'a> {
    /// Displayed to the client when the connection terminates
    pub reason: Chat<'a>,
}
