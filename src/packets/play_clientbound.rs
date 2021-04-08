use crate::nbt::NbtTag;

use super::*;

#[derive(MinecraftPacketPart)]
#[discriminant(VarInt)]
enum ClientBoundPacket<'a> {
    SpawnEntity {
        id: VarInt,
        uuid: UUID,
        entity_type: VarInt,
        x: f64,
        y: f64,
        z: f64,
        pitch: Angle,
        yaw: Angle,
        data: i32,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },

    SpawnExperienceOrb {
        id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        count: i16,
    },

    SpawnLivingEntity {
        id: VarInt,
        uuid: UUID,
        entity_type: VarInt,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        head_pitch: Angle,
        velocity_x: i16,
        velocity_y: i16,
        velocity_z: i16,
    },

    SpawnPainting {
        id: VarInt,
        uuid: UUID,
        motive: crate::paintings::Painting,
        location: Position,
        direction: Direction,
    },

    SpawnPlayer {
        id: VarInt,
        uuid: UUID,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
    },

    EntityAnimation {
        id: VarInt,
        animation: crate::animations::Animation,
    },

    Statistics {
        count: VarInt,
        statistic: RawBytes<'a>,
    },

    AcknowledgePlayerDigging {
        location: Position,
        block: VarInt,
        status: crate::blocks::PartialDiggingState,
        successful: bool,
    },

    /// 0–9 are the displayable destroy stages and each other number means that there is no animation on this coordinate.
    ///
    /// Block break animations can still be applied on air; the animation will remain visible although there is no block being broken. However, if this is applied to a transparent block, odd graphical effects may happen, including water losing its transparency. (An effect similar to this can be seen in normal gameplay when breaking ice blocks)
    ///
    /// If you need to display several break animations at the same time you have to give each of them a unique Entity ID. The entity ID does not need to correspond to an actual entity on the client. It is valid to use a randomly generated number.
    BlockBreakAnimation {
        /// Entity ID of the entity breaking the block
        id: VarInt,
        /// Block Position
        location: Position,
        /// 0–9 to set it, any other value to remove it
        destroy_stage: u8,
    },

    /// Sets the block entity associated with the block at the given location.
    BlockEntityData {
        location: Position,
        /// The type of update to perform, see [crate::blocks::BlockEntityDataAction].
        action: crate::blocks::BlockEntityDataAction,
        /// Data to set. May be [crate::nbt::NbtTag::Null], in which case the block entity at the given location is removed (though this is not required since the client will remove the block entity automatically on chunk unload or block removal).
        data: crate::nbt::NbtTag<'a>,
    },

    /// This packet is used for a number of actions and animations performed by blocks, usually non-persistent.
    ///
    /// See [Block Actions](https://wiki.vg/Block_Actions) for a list of values.
    ///
    /// **Warning**: This packet uses a block ID, not a block state.
    BlockAction {
        /// Block coordinates
        location: Position,
        /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
        action_id: u8,
        /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
        action_param: u8,
        /// The block type ID for the block. This must match the block at the given coordinates
        block_type: VarInt,
    },

    /// Fired whenever a block is changed within the render distance.
    /// Changes include plant growth, cake bites, redstone repeater delay changes, block facing changes (bed, chest, hopper...) and many other values depending on the type of the block.
    ///
    /// **Warning**: Changing a block in a chunk that is not loaded is not a stable action. The Notchian client currently uses a shared empty chunk which is modified for all block changes in unloaded chunks; while in 1.9 this chunk never renders in older versions the changed block will appear in all copies of the empty chunk. Servers should avoid sending block changes in unloaded chunks and clients should ignore such packets.
    BlockChange {
        /// Block Coordinates
        location: Position,
        /// The new block state ID for the block as given in the [global palette](http://minecraft.gamepedia.com/Data_values%23Block_IDs). See that section for more information.
        block_state: VarInt,
    },

    BossBar {
        /// Unique ID for this bar.
        uuid: UUID,
        /// The action to apply on the boss bar.
        action: crate::boss_bar::BossBarAction<'a>,
    },

    /// Changes the difficulty setting in the client's option menu
    ServerDifficulty {
        difficulty: crate::difficulty::Difficulty,
        difficulty_locked: bool,
    },

    /// Identifying the difference between Chat/System Message is important as it helps respect the user's chat visibility options. See [processing chat](https://wiki.vg/Chat#Processing_chat) for more info about these positions.
    ///
    /// **Warning**: Game info accepts json formatting but does not display it, although the deprecated §-based formatting works. This is not an issue when using the [Title] packet, so prefer that packet for displaying information in that slot. See MC-119145 for more information.
    ChatMessage {
        message: Chat<'a>,
        position: crate::chat::Position,
        /// Used by the Notchian client for the disableChat launch option. Setting 0 will always display the message regardless of the setting.
        sender: UUID,
    },

    /// The server responds with a list of auto-completions of the last word sent to it.
    /// In the case of regular chat, this is a player username.
    /// Command names and parameters are also supported.
    /// The client sorts these alphabetically before listing them.
    TabComplete {
        /// Transaction ID
        id: VarInt,
        /// Start of the text to replace
        start: VarInt,
        /// Length of the text to replace
        lenght: VarInt,
        /// Eligible values to insert, note that each command is sent separately instead of in a single string, hence the need for an [Array].
        matches: Array<'a, crate::auto_completion::Match<'a>, VarInt>,
    },

    /// Lists all of the commands on the server, and how they are parsed.
    /// This is a directed graph, with one root node. Each redirect or child node must refer only to nodes that have already been declared.
    DeclareCommands {
        count: VarInt,
        /// An array of [Node](https://wiki.vg/Command_Data) followed by the index of the `root` node in the array.
        /// Parsing is unimplemented yet.
        data: RawBytes<'a>,
    },

    /// A packet from the server indicating whether a request from the client was accepted, or whether there was a conflict (due to lag).
    /// If the packet was not accepted, the client must respond with a serverbound window confirmation packet.
    WindowConfirmation {
        /// The ID of the window that the action occurred in.
        window_id: u8,
        /// Every action that is to be accepted has a unique ID. This number is an incrementing integer (starting at 0) with separate counts for each window ID.
        action_id: i16,
        /// Whether the action was accepted.
        accepted: bool,
    },

    /// This packet is sent from the server to the client when a window is forcibly closed, such as when a chest is destroyed while it's open.
    CloseWindow {
        /// This is the ID of the window that was closed. 0 for inventory.
        window_id: u8,
    },

    /// Sent by the server when items in multiple slots (in a window) are added/removed.
    /// This includes the main inventory, equipped armour and crafting slots.
    WindowItems {
        /// The ID of window which items are being sent for. 0 for player inventory.
        window_id: u8,
        /// The [crate::slots::Slot]s in this window.
        /// See [inventory windows](https://wiki.vg/Inventory#Windows) for further information about how slots are indexed.
        slots: Array<'a, Option<crate::slots::Slot<'a>>, i16>,
    },

    /// This packet is used to inform the client that part of a GUI window should be updated.
    WindowProperty {
        window_id: u8,
        /// The property to be updated.
        /// The meaning of this field depends on the type of the window.
        /// The [the wiki](https://wiki.vg/Protocol#Window_Property) shows the known combinations of window type and property, and how the value is to be interpreted.
        property: i16,
        /// The new value for the property.
        /// The meaning of this field depends on the type of the window.
        /// The [the wiki](https://wiki.vg/Protocol#Window_Property) shows the known combinations of window type and property, and how the value is to be interpreted.
        value: i16,
    },

    /// Sent by the server when an item in a slot (in a window) is added/removed.
    ///
    /// To set the cursor (the item currently dragged with the mouse), use -1 as `window_id` and as `slot_index`.
    ///
    /// This packet can only be used to edit the hotbar of the player's inventory if window ID is set to 0 (slots 36 through 44). If the window ID is set to -2, then any slot in the inventory can be used but no add item animation will be played.
    SetSlot {
        /// The window which is being updated. 0 for player inventory.
        /// Note that all known window types include the player inventory.
        /// This packet will only be sent for the currently opened window while the player is performing actions, even if it affects the player inventory.
        /// After the window is closed, a number of these packets are sent to update the player's inventory window (0).
        window_id: i8,
        /// The slot that should be updated.
        slot_index: i16,
        slot_value: crate::slots::Slot<'a>,
    },

    /// Applies a cooldown period to all items with the given type.
    /// Used by the Notchian server with enderpearls.
    /// This packet should be sent when the cooldown starts and also when the cooldown ends (to compensate for lag), although the client will end the cooldown automatically.
    /// Can be applied to any item, note that interactions still get sent to the server with the item but the client does not play the animation nor attempt to predict results (i.e block placing).
    SetCooldown {
        /// Numeric ID of the item to apply a cooldown to.
        item_id: VarInt,
        /// Number of ticks to apply a cooldown for, or 0 to clear the cooldown.
        cooldown_ticks: VarInt,
    },

    /// Mods and plugins can use this to send their data.
    /// Minecraft itself uses several [plugin channels](https://wiki.vg/Plugin_channel).
    /// These internal channels are in the `minecraft` namespace.
    ///
    /// [More documentation](http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)
    PluginMessage {
        /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
        identifier: Identifier<'a>,
        /// Any data, depending on the channel.
        /// `minecraft:` channels are documented [here](https://wiki.vg/Plugin_channel).
        /// The length of this array must be inferred from the packet length.
        data: RawBytes<'a>,
    },

    /// Used to play a sound effect on the client.
    /// Custom sounds may be added by resource packs.
    NamedSoundEffect {
        /// All sound effect names as of 1.16.5 can be seen [here](https://pokechu22.github.io/Burger/1.16.5.html#sounds).
        sound_name: Identifier<'a>,
        /// The category that this sound will be played from ([current categories](https://gist.github.com/konwboj/7c0c380d3923443e9d55)).
        sound_category: VarInt,
        /// Effect X multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
        effect_position_x: i32,
        /// Effect Y multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
        effect_position_y: i32,
        /// Effect Z multiplied by 8 ([fixed-point number](https://wiki.vg/Data_types#Fixed-point_numbers) with only 3 bits dedicated to the fractional part).
        effect_position_z: i32,
        /// `1.0` is 100%, can be more.
        volume: f32,
        /// Float between 0.5 and 2.0 by Notchian clients.
        pitch: f32,
    },

    /// Sent by the server before it disconnects a client.
    /// The client assumes that the server has already closed the connection by the time the packet arrives.
    Disconnect {
        /// Displayed to the client when the connection terminates
        reason: Chat<'a>,
    },

    /// Entity statuses generally trigger an animation for an entity.
    /// The available statuses vary by the entity's type (and are available to subclasses of that type as well).
    EntityStatus {
        entity_id: i32,
        /// See [Entity statuses](https://wiki.vg/Entity_statuses) for a list of which statuses are valid for each type of entity.
        entity_status: u8,
    },

    /// Sent when an explosion occurs (creepers, TNT, and ghast fireballs).
    /// Each block in Records is set to air. Coordinates for each axis in record is int(X) + record.x
    Explosion {
        /// The coordinate x of the explosion
        x: f32,
        /// The coordinate y of the explosion
        y: f32,
        /// The coordinate z of the explosion
        z: f32,
        /// A strength greater than or equal to 2.0 spawns a `minecraft:explosion_emitter` particle, while a lesser strength spawns a `minecraft:explosion` particle.
        strenght: f32,
        /// Each record is 3 signed bytes long; the 3 bytes are the XYZ (respectively) signed offsets of affected blocks.
        affected_blocks: Array<'a, (i8, i8, i8), i32>,
        /// X velocity of the player being pushed by the explosion.
        player_acceleration_x: f32,
        /// Y velocity of the player being pushed by the explosion.
        player_acceleration_y: f32,
        /// Z velocity of the player being pushed by the explosion.
        player_acceleration_z: f32,
    },

    /// Tells the client to unload a chunk column.
    /// It is legal to send this packet even if the given chunk is not currently loaded.
    UnloadChunk {
        /// Block coordinate divided by 16, rounded down.
        chunk_x: i32,
        /// Block coordinate divided by 16, rounded down.
        chunk_y: i32,
    },

    /// Used for a wide variety of game state things, from weather to bed use to gamemode to demo messages.
    ChangeGameState {
        /// The type of change
        reason: crate::game_state::GameState,
        /// The meaning of this value depends on the `reason` field.
        value: f32,
    },

    /// This packet is used exclusively for opening the horse GUI.
    /// [Self::OpenWindow] is used for all other GUIs.
    OpenHorseWindow {
        window_id: u8,
        slot_count: VarInt,
        entity_id: i32,
    },

    /// The server will frequently send out a keep-alive, each containing a random ID.
    /// The client must respond with the same payload (see [serverbound Keep Alive](https://wiki.vg/Protocol#Keep_Alive_.28serverbound.29)).
    /// If the client does not respond to them for over 30 seconds, the server kicks the client.
    /// Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yields a "Timed out" exception.
    KeepAlive {
        /// The Notchian server uses a system-dependent time in milliseconds to generate the keep alive ID value.
        keep_alive_id: i64,
    },

    ChunkData {
        value: crate::chunk::ChunkData<'a>,
    },

    /// Sent when a client is to play a sound or particle effect.
    Effect {
        effect_id: crate::animations::Effect,
        /// The location of the effect
        location: Position,
        /// Extra data for certain effects, see [the wiki](https://wiki.vg/Protocol#Effect)
        data: i32,
        /// By default, the Minecraft client adjusts the volume of sound effects based on distance.
        /// The final boolean field is used to disable this, and instead the effect is played from 2 blocks away in the correct direction.
        /// Currently this is only used for [crate::animations::Effect::WitherSpawn], [crate::animations::Effect::EnderdragonDeath], and [crate::animations::Effect::EndPortalOpening]; it is ignored on other effects.
        disable_relative_volume: bool,
    },

    /// Displays the named particle
    Particle {
        /// The particle ID listed in the [particle data type](https://wiki.vg/Protocol#Particle)
        particle_id: i32,
        /// If true, particle distance increases from 256 to 65536.
        long_distance: bool,
        /// X position of the particle
        x: f64,
        /// Y position of the particle
        y: f64,
        /// Z position of the particle
        z: f64,
        /// This is added to the X position after being multiplied by `random.nextGaussian()`.
        offset_x: f32,
        /// This is added to the Y position after being multiplied by `random.nextGaussian()`.
        offset_y: f32,
        /// This is added to the Z position after being multiplied by `random.nextGaussian()`.
        offset_z: f32,
        /// The data of each particle
        particule_data: f32,
        /// The number of particles to create
        particule_count: i32,
        /// The variable data listed in the [particle data type](https://wiki.vg/Protocol#Particle)
        data: RawBytes<'a>,
    },

    /// Updates light levels for a chunk
    UpdateLight {
        /// TODO: parse this
        data: RawBytes<'a>,
    },

    /// See [Protocol Encryption](https://wiki.vg/Protocol_Encryption) for information on logging in.
    JoinGame {
        /// The player's Entity ID (EID)
        player_id: i32,
        is_harcore: bool,
        gamemode: crate::gamemode::Gamemode,
        previous_gamemode: crate::gamemode::PreviousGamemode,
        /// Identifiers for all worlds on the server
        worlds_names: Array<'a, Identifier<'a>, VarInt>,
        /// The full extent of these is still unknown, but the tag represents a dimension and biome registry.
        /// See [the wiki](https://wiki.vg/Protocol#Join_Game) for the vanilla default.
        dimension_coded: NbtTag<'a>,
        /// Valid dimensions are defined per dimension registry sent before this.
        /// The structure of this tag is a dimension type (see [the wiki](https://wiki.vg/Protocol#Join_Game)).
        dimension: NbtTag<'a>,
        /// Name of the world being spawned into
        world_name: Identifier<'a>,
        /// First 8 bytes of the SHA-256 hash of the world's seed.
        /// Used client side for biome noise.
        hashed_seed: i64,
        /// Was once used by the client to draw the player list, but now is ignored.
        max_players: VarInt,
        /// Render distance (2..32).
        render_distance: VarInt,
        /// If `true`, a Notchian client shows reduced information on the debug screen.
        /// For servers in development, this should almost always be `false`.
        reduced_debug_info: bool,
        /// Set to false when the `doImmediateRespawn` gamerule is `true`.
        enable_respawn_screen: bool,
        /// `true` if the world is a [debug mode world](http://minecraft.gamepedia.com/Debug_mode); debug mode worlds cannot be modified and have predefined blocks.
        is_debug: bool,
        /// `true` if the world is a [superflat world](http://minecraft.gamepedia.com/Superflat); flat worlds have different void fog and a horizon at y=0 instead of y=63.
        is_flat: bool,
    },

    /// Updates a rectangular area on a map item
    MapData {
        /// TODO: parse this
        data: RawBytes<'a>,
    },

    /// Lists the trades a villager NPC is offering
    TradeList {
        /// The ID of the window that is open
        window_id: VarInt,
        /// The list of trades a villager NPC is offering
        trades: Array<'a, crate::trades::Trade<'a>, u8>,
        /// The villager appearance
        villager_level: crate::trades::VillagerLevel,
        /// Total experience for this villager (always 0 for the wandering trader)
        experience: VarInt,
        /// True if this is a regular villager; false for the wandering trader.
        /// When false, hides the villager level and some other GUI elements.
        is_regular_villager: bool,
        /// True for regular villagers and false for the wandering trader.
        /// If true, the "Villagers restock up to two times per day." message is displayed when hovering over disabled trades.
        can_restock: bool,
    },

    /// This packet is sent by the server when an entity moves less then 8 blocks; if an entity moves more than 8 blocks Entity Teleport should be sent instead.
    /// This packet allows at most 8 blocks movement in any direction, because `i16` range is from -32768 to 32767. And 32768 / (128 * 32) = 8.
    EntityPosition {
        entity_id: VarInt,
        /// Change in X position as `(currentX * 32 - prevX * 32) * 128`
        delta_x: i16,
        /// Change in Y position as `(currentX * 32 - prevX * 32) * 128`
        delta_y: i16,
        /// Change in Z position as `(currentX * 32 - prevX * 32) * 128`
        delta_z: i16,
        on_ground: bool,
    },

    /// This packet is sent by the server when an entity rotates and moves.
    /// Since a `i16` range is limited from -32768 to 32767, and movement is offset of fixed-point numbers, this packet allows at most 8 blocks movement in any direction. `(-32768 / (32 * 128) == -8)`
    EntityPositionAndRotation {
        entity_id: VarInt,
        /// Change in X position as `(currentX * 32 - prevX * 32) * 128`
        delta_x: i16,
        /// Change in Y position as `(currentX * 32 - prevX * 32) * 128`
        delta_y: i16,
        /// Change in Z position as `(currentX * 32 - prevX * 32) * 128`
        delta_z: i16,
        /// New angle, not a delta
        yaw: Angle,
        /// New angle, not a delta
        pitch: Angle,
        on_ground: bool,
    },

    /// This packet is sent by the server when an entity rotates.
    EntityRotation {
        entity_id: VarInt,
        /// New angle, not a delta
        yaw: Angle,
        /// New angle, not a delta
        pitch: Angle,
        on_ground: bool,
    },

    /// This packet may be used to initialize an entity.
    /// 
    /// For player entities, either this packet or any move/look packet is sent every game tick.
    /// So the meaning of this packet is basically that the entity did not move/look since the last such packet.
    EntityMovement {
        entity_id: VarInt,
    },

    /// Note that all fields use absolute positioning and do not allow for relative positioning.
    VehicleMove {
        /// Absolute position (X coordinate)
        x: f64,
        /// Absolute position (Y coordinate)
        y: f64,
        /// Absolute position (Z coordinate)
        z: f64,
        /// Absolute rotation on the vertical axis, in degrees
        yaw: f32,
        /// Absolute rotation on the horizontal axis, in degrees
        pitch: f32,
    },

    /// Sent when a player right clicks with a signed book.
    /// This tells the client to open the book GUI.
    OpenBook {
        hand: crate::slots::Hand,
    },
}
