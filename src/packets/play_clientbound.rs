#[allow(unused_imports)]
use super::play_serverbound::ServerboundPacket;
use crate::nbt::NbtTag;
use crate::components::*;
use crate::ids::*;
use crate::ids::blocks;
use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ClientboundPacket<'a> {
    /// Sent by the server when a vehicle or other **non-living entity** is created
    SpawnEntity {
        id: VarInt,
        uuid: UUID,
        entity_type: entities::Entity,
        x: f64,
        y: f64,
        z: f64,
        pitch: Angle,
        yaw: Angle,
        /// Meaning dependent on the value of the `type` field, see [Object Data](https://wiki.vg/Object_Data) for details.
        data: i32,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_x: i16,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_y: i16,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_z: i16,
    },

    /// Spawns one or more experience orbs
    SpawnExperienceOrb {
        id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        /// The amount of experience this orb will reward once collected.
        count: i16,
    },

    /// Sent by the server when a living entity is spawned
    SpawnLivingEntity {
        id: VarInt,
        uuid: UUID,
        entity_type: entities::Entity,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
        head_pitch: Angle,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_x: i16,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_y: i16,
        /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
        velocity_z: i16,
    },

    /// This packet shows location, name, and type of painting.
    SpawnPainting {
        id: VarInt,
        uuid: UUID,
        motive: paintings::Painting,
        /// Center coordinates
        location: Position,
        /// Direction the painting faces
        direction: Direction,
    },

    // todo add doc links
    /// This packet is sent by the server when a player comes into visible range, not when a player joins.
    ///
    /// This packet must be sent after the Player Info packet that adds the player data for the client to use when spawning a player.
    /// If the Player Info for the player spawned by this packet is not present when this packet arrives, Notchian clients will not spawn the player entity.
    /// The Player Info packet includes skin/cape data.
    ///
    /// Servers can, however, safely spawn player entities for players not in visible range.
    /// The client appears to handle it correctly.
    SpawnPlayer {
        id: VarInt,
        uuid: UUID,
        x: f64,
        y: f64,
        z: f64,
        yaw: Angle,
        pitch: Angle,
    },

    /// Shows a permanent particle.
    SculkVibrationSignal {
        /// Source position for the vibration
        source_position: Position,
        /// Identifier of the destination codec type
        destination_identifier: Identifier<'a>,
        rest: RawBytes<'a>,
    },

    /// Sent whenever an entity should change animation
    EntityAnimation {
        id: VarInt,
        animation: animations::Animation,
    },

    /// Will only send the changed values if previously requested.
    ///
    /// *Response to [ServerboundPacket::ClientStatus]*
    Statistics {
        statistics: Array<'a, advancements::Statistic, VarInt>,
    },

    AcknowledgePlayerDigging {
        /// Position where the digging was happening
        location: Position,
        /// Block state ID of the block that should be at that position now.
        /// Use [Block::from_state_id](blocks::Block::from_state_id) to get the corresponding [Block](blocks::Block).
        block: VarInt,
        status: crate::components::blocks::PartialDiggingState,
        /// True if the digging succeeded; false if the client should undo any changes it made locally.
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
        /// The type of update to perform, see [crate::components::blocks::BlockEntityDataAction].
        action: crate::components::blocks::BlockEntityDataAction,
        /// Data to set. May be [nbt::NbtTag::Null], in which case the block entity at the given location is removed (though this is not required since the client will remove the block entity automatically on chunk unload or block removal).
        data: NbtTag,
    },

    /// This packet is used for a number of actions and animations performed by blocks, usually non-persistent.
    ///
    /// See [Block Actions](https://wiki.vg/Block_Actions) for a list of values.
    BlockAction {
        /// Block coordinates
        location: Position,
        /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
        action_id: u8,
        /// Varies depending on block — see [Block Actions](https://wiki.vg/Block_Actions)
        action_param: u8,
        /// The block ID. This must match the block at the given coordinates
        block: blocks::Block,
    },

    /// Fired whenever a block is changed within the render distance.
    /// Changes include plant growth, cake bites, redstone repeater delay changes, block facing changes (bed, chest, hopper...) and many other values depending on the type of the block.
    ///
    /// **Warning**: Changing a block in a chunk that is not loaded is not a stable action. The Notchian client currently uses a shared empty chunk which is modified for all block changes in unloaded chunks; while in 1.9 this chunk never renders in older versions the changed block will appear in all copies of the empty chunk. Servers should avoid sending block changes in unloaded chunks and clients should ignore such packets.
    BlockChange {
        /// Block Coordinates
        location: Position,
        /// The new block state ID for the block as given in the [global palette](http://minecraft.gamepedia.com/Data_values%23Block_IDs). See that section for more information.
        /// Use [Block::from_state_id](blocks::Block::from_state_id) to get the corresponding [Block](blocks::Block).
        block_state: VarInt,
    },

    BossBar {
        /// Unique ID for this bar.
        uuid: UUID,
        /// The action to apply on the boss bar.
        action: boss_bar::BossBarAction<'a>,
    },

    /// Changes the difficulty setting in the client's option menu
    ServerDifficulty {
        difficulty: difficulty::Difficulty,
        difficulty_locked: bool,
    },

    /// Identifying the difference between Chat/System Message is important as it helps respect the user's chat visibility options. See [processing chat](https://wiki.vg/Chat#Processing_chat) for more info about these positions.
    ///
    /// **Warning**: Game info accepts json formatting but does not display it, although the deprecated §-based formatting works. This is not an issue when using the [Title] packet, so prefer that packet for displaying information in that slot. See MC-119145 for more information.
    ///
    /// *See also [ServerboundPacket::ChatMessage]*
    ChatMessage {
        message: Chat<'a>,
        position: chat::Position,
        /// Used by the Notchian client for the disableChat launch option. Setting 0 will always display the message regardless of the setting.
        sender: UUID,
    },

    /// Clears the client's current title information, with the option to also reset it.
    ClearTitles {
        reset: bool,
    },

    /// The server responds with a list of auto-completions of the last word sent to it.
    /// In the case of regular chat, this is a player username.
    /// Command names and parameters are also supported.
    /// The client sorts these alphabetically before listing them.
    ///
    /// *Response to [ClientboundPacket::TabComplete]*
    TabComplete {
        transaction_id: VarInt,
        /// Start of the text to replace
        start: VarInt,
        /// Length of the text to replace
        lenght: VarInt,
        /// Eligible values to insert, note that each command is sent separately instead of in a single string, hence the need for an [Array].
        matches: Array<'a, auto_completion::Match<'a>, VarInt>,
    },

    /// Lists all of the commands on the server, and how they are parsed.
    /// This is a directed graph, with one root node. Each redirect or child node must refer only to nodes that have already been declared.
    DeclareCommands {
        count: VarInt,
        /// An array of [Node](https://wiki.vg/Command_Data) followed by the index of the `root` node in the array.
        /// Parsing is unimplemented yet.
        data: RawBytes<'a>,
    },

    /// This packet is sent from the server to the client when a window is forcibly closed, such as when a chest is destroyed while it's open.
    CloseWindow {
        /// This is the ID of the window that was closed. 0 for inventory.
        window_id: i8,
    },

    /// Sent by the server when items in multiple slots (in a window) are added/removed.
    /// This includes the main inventory, equipped armour and crafting slots.
    WindowItems {
        /// The ID of window which items are being sent for. 0 for player inventory.
        window_id: i8,
        /// The [slots::Slot]s in this window.
        /// See [inventory windows](https://wiki.vg/Inventory#Windows) for further information about how slots are indexed.
        slots: Array<'a, slots::Slot, i16>,
    },

    /// This packet is used to inform the client that part of a GUI window should be updated.
    WindowProperty {
        window_id: i8,
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
        slot_value: slots::Slot,
    },

    /// Applies a cooldown period to all items with the given type.
    /// Used by the Notchian server with enderpearls.
    /// This packet should be sent when the cooldown starts and also when the cooldown ends (to compensate for lag), although the client will end the cooldown automatically.
    /// Can be applied to any item, note that interactions still get sent to the server with the item but the client does not play the animation nor attempt to predict results (i.e block placing).
    SetCooldown {
        /// The item to apply a cooldown to.
        item: items::Item,
        /// Number of ticks to apply a cooldown for, or 0 to clear the cooldown.
        cooldown_ticks: VarInt,
    },

    /// Mods and plugins can use this to send their data.
    /// Minecraft itself uses several [plugin channels](https://wiki.vg/Plugin_channel).
    /// These internal channels are in the `minecraft` namespace.
    ///
    /// [More documentation](http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)
    ///
    /// *See also [ServerboundPacket::PluginMessage]*
    PluginMessage {
        /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
        identifier: Identifier<'a>,
        /// Any data, depending on the channel.
        /// `minecraft:` channels are documented [here](https://wiki.vg/Plugin_channel).
        data: RawBytes<'a>,
    },

    /// Used to play a sound effect on the client.
    /// Custom sounds may be added by resource packs.
    NamedSoundEffect {
        /// All sound effect names as of 1.16.5 can be seen [here](https://pokechu22.github.io/Burger/1.16.5.html#sounds).
        sound_name: Identifier<'a>,
        sound_category: sound::SoundCategory,
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
        reason: game_state::GameState,
        /// The meaning of this value depends on the `reason` field.
        value: f32,
    },

    /// This packet is used exclusively for opening the horse GUI.
    /// [Self::OpenWindow] is used for all other GUIs.
    OpenHorseWindow {
        window_id: i8,
        slot_count: VarInt,
        entity_id: i32,
    },

    /// The Notchian client determines how solid to display the warning by comparing to whichever is higher, the warning distance or whichever is lower, the distance from the current diameter to the target diameter or the place the border will be after warningTime seconds.
    IntitializeWorldBorder {
        x: f64,
        y: f64,
        /// Current length of a single side of the world border, in meters.
        old_diameter: f64,
        /// Target length of a single side of the world border, in meters.
        new_diameter: f64,
        /// Number of real-time milliseconds until New Diameter is reached.
        /// It appears that Notchian server does not sync world border speed to game ticks, so it gets out of sync with server lag.
        /// If the world border is not moving, this is set to 0.
        speed: VarLong,
        /// Resulting coordinates from a portal teleport are limited to ±value.
        /// Usually 29999984.
        portal_teleport_boundary: VarInt,
        /// In meters
        warning_blocks: VarInt,
        /// In seconds as set by `/worldborder` warning time.
        warning_time: VarInt,
    },

    /// The server will frequently send out a keep-alive, each containing a random ID.
    /// The client must respond with the same payload (see [serverbound Keep Alive](https://wiki.vg/Protocol#Keep_Alive_.28serverbound.29)).
    /// If the client does not respond to them for over 30 seconds, the server kicks the client.
    /// Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yields a "Timed out" exception.
    ///
    /// *Request for [ServerboundPacket::KeepAlive]*
    KeepAlive {
        /// The Notchian server uses a system-dependent time in milliseconds to generate the keep alive ID value.
        keep_alive_id: u64,
    },

    ChunkData {
        value: chunk::ChunkData<'a>,
    },

    /// Sent when a client is to play a sound or particle effect.
    Effect {
        effect_id: animations::Effect,
        /// The location of the effect
        location: Position,
        /// Extra data for certain effects, see [the wiki](https://wiki.vg/Protocol#Effect)
        data: i32,
        /// By default, the Minecraft client adjusts the volume of sound effects based on distance.
        /// The final boolean field is used to disable this, and instead the effect is played from 2 blocks away in the correct direction.
        /// Currently this is only used for [animations::Effect::WitherSpawn], [animations::Effect::EnderdragonDeath], and [animations::Effect::EndPortalOpening]; it is ignored on other effects.
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
        is_hardcore: bool,
        gamemode: gamemode::Gamemode,
        previous_gamemode: gamemode::PreviousGamemode,
        /// Identifiers for all worlds on the server
        worlds_names: Array<'a, Identifier<'a>, VarInt>,
        /// The full extent of these is still unknown, but the tag represents a dimension and biome registry.
        /// See [the wiki](https://wiki.vg/Protocol#Join_Game) for the vanilla default.
        dimension_coded: NbtTag,
        /// Valid dimensions are defined per dimension registry sent before this.
        /// The structure of this tag is a dimension type (see [the wiki](https://wiki.vg/Protocol#Join_Game)).
        dimension: NbtTag,
        /// Name of the world being spawned into
        world_name: Identifier<'a>,
        /// First 8 bytes of the SHA-256 hash of the world's seed.
        /// Used client side for biome noise.
        hashed_seed: u64,
        /// Was once used by the client to draw the player list, but now is ignored.
        max_players: VarInt,
        /// Render distance (2..=32).
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

    /// Updates a rectangular area on a map **item**
    MapData {
        /// TODO: parse this
        data: RawBytes<'a>,
    },

    /// Lists the trades a villager NPC is offering
    TradeList {
        /// The ID of the window that is open
        window_id: VarInt,
        /// The list of trades a villager NPC is offering
        trades: Array<'a, trades::Trade, u8>,
        /// The villager appearance
        villager_level: trades::VillagerLevel,
        /// Total experience for this villager (always 0 for the wandering trader)
        experience: VarInt,
        /// True if this is a regular villager; false for the wandering trader.
        /// When false, hides the villager level and some other GUI elements.
        is_regular_villager: bool,
        /// True for regular villagers and false for the wandering trader.
        /// If true, the "Villagers restock up to two times per day." message is displayed when hovering over disabled trades.
        can_restock: bool,
    },

    /// This packet is sent by the server when an entity moves less then 8 blocks; if an entity moves more than 8 blocks [ClientboundPacket::TeleportEntity] should be sent instead.
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
        hand: slots::Hand,
    },

    /// This is sent to the client when it should open an inventory, such as a chest, workbench, or furnace.
    /// This message is not sent anywhere for clients opening their own inventory.
    /// For horses, use [ClientboundPacket::OpenHorseWindow].
    OpenWindow {
        /// A unique id number for the window to be displayed.
        /// Notchian server implementation is a counter, starting at 1.
        window_id: VarInt,
        /// The window type to use for display.
        /// TODO: replace by an enum
        window_type: VarInt,
        /// The title of the window
        window_title: Chat<'a>,
    },

    /// Sent when the client has placed a sign and is allowed to send [ServerboundPacket::UpdateSign].
    /// There must already be a sign at the given location (which the client does not do automatically) - send a [ClientboundPacket::BlockChange] first.
    ///
    /// *Request for [ServerboundPacket::UpdateSign]*
    OpenSignEditor {
        location: Position,
    },

    /// Unknown what this packet does just yet, not used by the Notchian server or client.
    /// Most likely added as a replacement to the removed window confirmation packet.
    UselessPacket {
        id: i32,
    },

    // Todo make add doc links
    /// Response to the serverbound packet (Craft Recipe Request), with the same recipe ID.
    /// Appears to be used to notify the UI.
    CraftRecipeResponse {
        window_id: i8,
        /// A recipe ID
        recipe: Identifier<'a>,
    },

    PlayerAbilities {
        /// Bit field, see [the wiki](https://wiki.vg/Protocol#Player_Abilities_.28clientbound.29).
        flags: u8,
        /// 0.05 by default
        flying_speed: f32,
        /// Modifies the field of view, like a speed potion.
        /// A Notchian server will use the same value as the movement speed sent in the Entity Properties packet, which defaults to 0.1 for players.
        field_of_view_modifier: f32,
    },

    /// Unused by the Notchain client.
    /// This data was once used for twitch.tv metadata circa 1.8.
    EndCombatEvent {
        /// Length of the combat in ticks.
        duration: VarInt,
        /// ID of the primary opponent of the ended combat, or -1 if there is no obvious primary opponent.
        entity_id: i32,
    },

    /// Unused by the Notchain client.
    /// This data was once used for twitch.tv metadata circa 1.8.
    EnterCombatEvent,

    /// Used to send a respawn screen.
    DeathCombatEvent {
        /// Entity ID of the player that died (should match the client's entity ID)
        player_id: VarInt,
        /// The killing entity's ID, or -1 if there is no obvious killer
        entity_id: i32,
        /// The death message
        message: Chat<'a>,
    },

    /// Sent by the server to update the user list (<tab> in the client).
    PlayerInfo {
        value: players::PlayerInfoAction<'a>,
    },

    /// Used to rotate the client player to face the given location or entity (for `/teleport [<targets>] <x> <y> <z> facing`)
    FacePlayer {
        /// If set to eyes, aims using the head position; otherwise aims using the feet position
        aim: players::FaceAim,
        /// X coordinate of the point to face towards
        target_x: f64,
        /// Y coordinate of the point to face towards
        target_y: f64,
        /// Z coordinate of the point to face towards
        target_z: f64,
        /// Used to reference a targeted entity.
        /// If the entity target appears invalid, it should be ignored.
        target: Option<players::FaceTarget>,
    },

    /// Updates the player's position on the server.
    /// This packet will also close the “Downloading Terrain” screen when joining/respawning.
    ///
    /// If the distance between the last known position of the player on the server and the new position set by this packet is greater than 100 meters, the client will be kicked for `You moved too quickly`.
    ///
    /// *Request for [ServerboundPacket::TeleportConfirm]*
    PlayerPositionAndLook {
        /// Absolute or relative position, depending on the `flags` field. If the last bit (`0b00000001`) is set, this value is relative.
        x: f64,
        /// Absolute or relative position, depending on the `flags` field. If the senventh bit (`0b00000010`) is set, this value is relative.
        y: f64,
        /// Absolute or relative position, depending on the `flags` field. If the sixth bit (`0b00000100`) is set, this value is relative.
        z: f64,
        /// Absolute or relative rotation on the X axis, depending on the `flags` field. If the fourth bit (`0b00010000`) is set, this value is relative.
        ///
        /// Yaw is measured in degrees, and does not follow classical trigonometry rules.
        /// The unit circle of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at (0, -1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any number is valid, including negative numbers and numbers greater than 360.
        yaw: f32,
        /// Absolute or relative rotation on the Y axis, depending on the `flags` field. If the fifth bit (`0b00001000`) is set, this value is relative.
        ///
        /// Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up, and 90 is looking straight down.
        pitch: f32,
        flags: u8,
        teleport_id: VarInt,
    },

    UnlockRecipes {
        action: recipes::UnlockRecipesAction<'a>,
    },

    /// Sent by the server when a list of entities is to be destroyed on the client
    DestroyEntities {
        entity_ids: Array<'a, VarInt, VarInt>,
    },

    RemoveEntityEffect {
        entity_id: VarInt,
        effect: effect::Effect,
    },

    /// *Request for [ServerboundPacket::ResourcePackStatus]*
    ResourcePackSend {
        /// The URL to the resource pack
        url: &'a str,
        /// A 40 character hexadecimal and lowercase SHA-1 hash of the resource pack file. (must be lower case in order to work)
        /// If it's not a 40 character hexadecimal string, the client will not use it for hash verification and likely waste bandwidth — but it will still treat it as a unique id.
        hash: &'a str,
    },

    /// To change the player's dimension (overworld/nether/end), send them a respawn packet with the appropriate dimension, followed by prechunks/chunks for the new dimension, and finally a position and look packet.
    /// You do not need to unload chunks, the client will do it automatically.
    ///
    /// **Warning**: Avoid changing player's dimension to same dimension they were already in unless they are dead.
    /// If you change the dimension to one they are already in, weird bugs can occur, such as the player being unable to attack other players in new world (until they die and respawn).
    /// If you must respawn a player in the same dimension without killing them, send two respawn packets, one to a different world and then another to the world you want.
    /// You do not need to complete the first respawn; it only matters that you send two packets.
    Respawn {
        /// Valid dimensions are defined per dimension registry sent in [ClientboundPacket::JoinGame].
        dimension: NbtTag,
        /// Name of the world being spawned into
        world_name: Identifier<'a>,
        /// First 8 bytes of the SHA-256 hash of the world's seed.
        /// Used client side for biome noise.
        hashed_seed: u64,
        gamemode: gamemode::Gamemode,
        previous_gamemode: gamemode::PreviousGamemode,
        /// `true` if the world is a [debug mode world](http://minecraft.gamepedia.com/Debug_mode); debug mode worlds cannot be modified and have predefined blocks.
        is_debug: bool,
        /// `true` if the world is a [superflat world](http://minecraft.gamepedia.com/Superflat); flat worlds have different void fog and a horizon at y=0 instead of y=63.
        is_flat: bool,
        /// If false, metadata is reset on the respawned player entity.
        /// Set to true for dimension changes (including the dimension change triggered by sending client status perform respawn to exit the end poem/credits), and false for normal respawns.
        copy_metadata: bool,
    },

    /// Changes the direction an entity's head is facing.
    ///
    /// While sending the [ClientboundPacket::EntityLook] packet changes the vertical rotation of the head, sending this packet appears to be necessary to rotate the head horizontally.
    EntityHeadLook {
        entity_id: VarInt,
        /// New angle, not a delta
        head_yew: Angle,
    },

    /// Fired whenever 2 or more blocks are changed within the same chunk on the same tick.
    ///
    /// **Warnin**: Changing blocks in chunks not loaded by the client is unsafe.
    MultiBlockChange {
        value: crate::components::blocks::MultiBlockChange<'a>,
    },

    /// Sent by the server to indicate that the client should switch advancement tab.
    /// Sent either when the client switches tab in the GUI or when an advancement in another tab is made.
    SelectAdvancementTab {
        /// The Identifier can be one of the following:
        /// - "minecraft:story/root"
        /// - "minecraft:nether/root"
        /// - "minecraft:end/root"
        /// - "minecraft:adventure/root"
        /// - "minecraft:husbandry/root"
        ///
        /// If no or an invalid identifier is sent, the client will switch to the first tab in the GUI.
        identifier: Option<Identifier<'a>>,
    },

    /// Displays a message above the hotbar (the same as position 2 in Chat Message (clientbound).
    ActionBar {
        action_bar_text: Chat<'a>,
    },

    /// Sets the entity that the player renders from.
    /// This is normally used when the player left-clicks an entity while in spectator mode.
    ///
    /// The player's camera will move with the entity and look where it is looking.
    /// The entity is often another player, but can be any type of entity.
    /// The player is unable to move this entity (move packets will act as if they are coming from the other entity).
    /// To return control to the player, send this packet with their entity ID.
    ///
    /// The Notchian server resets this (sends it back to the default entity) whenever the spectated entity is killed or the player sneaks, but only if they were spectating an entity.
    /// It also sends this packet whenever the player switches out of spectator mode (even if they weren't spectating an entity).
    Camera {
        /// ID of the entity to set the client's camera to.
        /// If the given entity is not loaded by the player, this packet is ignored.
        camera_id: VarInt,
    },

    /// Sent to change the player's slot selection
    ///
    /// *See also [ServerboundPacket::HeldItemChange]*
    HeldItemChange {
        /// The slot which the player has selected (0–8)
        slot: u8,
    },

    /// Updates the client's location.
    /// This is used to determine what chunks should remain loaded and if a chunk load should be ignored; chunks outside of the view distance may be unloaded.
    /// Sent whenever the player moves across a chunk border horizontally, and also (according to testing) for any integer change in the vertical axis, even if it doesn't go across a chunk section border.
    UpdateViewPosition {
        chunk_x: VarInt,
        chunk_z: VarInt,
    },

    /// Sent by the integrated singleplayer server when changing render distance.
    /// Does not appear to be used by the dedicated server, as view-distance in server.properties cannot be changed at runtime.
    UpdateViewDistance {
        /// Render distance (2..=32)
        view_distance: VarInt,
    },

    /// Sent by the server after login to specify the coordinates of the spawn point (the point at which players spawn at, and which the compass points to).
    /// It can be sent at any time to update the point compasses point at.
    SpawnPosition {
        location: Position,
    },

    /// This is sent to the client when it should display a scoreboard
    DisplayScoreboard {
        /// The position of the scoreboard
        position: teams::ScoreboardPosition,
        /// The unique name for the scoreboard to be displayed
        name: &'a str,
    },

    /// Updates one or more metadata properties for an existing entity.
    /// Any properties not included in the Metadata field are left unchanged.
    EntityMetadata {
        entity_id: VarInt,
        metadata: entity::EntityMetadata,
    },

    /// This packet is sent when an entity has been leashed to another entity.
    AttachEntity {
        /// Attached entity's EID.
        attached_entity_id: i32,
        /// ID of the entity holding the lead. Set to -1 to detach.
        holding_entity_id: i32,
    },

    /// Sets the velocity of an entity
    ///
    /// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example, -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
    EntityVelocity {
        entity_id: VarInt,
        /// Velocity on the X axis
        velocity_x: i16,
        /// Velocity on the Y axis
        velocity_y: i16,
        /// Velocity on the Z axis
        velocity_z: i16,
    },

    EntityEquipment {
        entity_id: VarInt,
        equipment: slots::EquipmentSlotArray,
    },

    /// Sent by the server when the client should change experience levels
    SetExperience {
        /// Between 0.0 and 1.0
        experience_bar: f32,
        experience_level: VarInt,
        /// See [Experience#Leveling up on the Minecraft Wiki](http://minecraft.gamepedia.com/Experience%23Leveling_up) for `total_experience` to `experience_level`.
        total_experience: VarInt,
    },

    /// Sent by the server to update/set the health of the player it is sent to
    UpdateHealth {
        /// 0.0 or less = dead, 20.0 = full HP
        health: f32,
        /// 0–20
        food: VarInt,
        /// Seems to vary from 0.0 to 5.0 in integer increments.
        /// Food saturation acts as a food “overcharge”.
        /// Food values will not decrease while the saturation is over zero.
        /// Players logging in automatically get a saturation of 5.0.
        /// Eating food increases the saturation as well as the food bar.
        food_saturation: f32,
    },

    /// This is sent to the client when it should create a new scoreboard objective or remove one
    ScoreboardObjective {
        /// A unique name for the objective
        objective_name: &'a str,
        action: teams::ScoreboardAction<'a>,
    },

    SetPassagers {
        vehicle_entity_id: VarInt,
        /// The IDs of entity's passengers
        passagers: Array<'a, VarInt, VarInt>,
    },

    /// Creates and updates teams
    Teams {
        /// A unique name for the team (Shared with scoreboard)
        team_name: &'a str,
        action: teams::TeamAction<'a>,
    },

    /// This is sent to the client when it should update a scoreboard item
    UpdateScore {
        /// The entity whose score this is. For players, this is their username; for other entities, it is their UUID.
        entity_name: &'a str,
        score_action: teams::ScoreboardScoreAction<'a>,
    },

    /// Time is based on ticks, where 20 ticks happen every second.
    /// There are 24000 ticks in a day, making Minecraft days exactly 20 minutes long.
    /// The time of day is based on the timestamp modulo 24000. 0 is sunrise, 6000 is noon, 12000 is sunset, and 18000 is midnight.
    /// The default SMP server increments the time by 20 every second.
    TimeUpdate {
        /// In ticks; not changed by server commands
        world_age: i64,
        /// The world (or region) time, in ticks.
        /// If negative the sun will stop moving.
        time_of_day: i64,
    },

    Title {
        action: chat::TitleAction<'a>,
    },

    /// Plays a sound effect from an entity
    EntitySoundEffect {
        /// ID of hardcoded sound event ([events](https://pokechu22.github.io/Burger/1.16.5.html#sounds) as of 1.16.5).
        /// TODO: generate an enum
        sound_id: VarInt,
        sound_category: sound::SoundCategory,
        entity_id: VarInt,
        /// 1.0 is 100%, capped between 0.0 and 1.0 by Notchian clients
        volume: f32,
        /// Float between 0.5 and 2.0 by Notchian clients
        pitch: f32,
    },

    /// This packet is used to play sound events with hardcoded IDs.
    ///
    /// Numeric sound effect IDs are liable to change between versions.
    /// For custom sounds, use [ClientboundPacket::NamedSoundEffect].
    SoundEffect {
        /// ID of hardcoded sound event ([events](https://pokechu22.github.io/Burger/1.16.5.html#sounds) as of 1.16.5).
        /// TODO: generate an enum
        sound_id: VarInt,
        sound_category: sound::SoundCategory,
        /// Effect X multiplied by 8 (fixed-point number with only 3 bits dedicated to the fractional part)
        effect_x: i32,
        /// Effect Y multiplied by 8 (fixed-point number with only 3 bits dedicated to the fractional part)
        effect_y: i32,
        /// Effect Z multiplied by 8 (fixed-point number with only 3 bits dedicated to the fractional part)
        effect_z: i32,
        /// 1.0 is 100%, capped between 0.0 and 1.0 by Notchian clients
        volume: f32,
        /// Float between 0.5 and 2.0 by Notchian clients
        pitch: f32,
    },

    StopSound {
        value: sound::StopSoundPacket<'a>,
    },

    /// This packet may be used by custom servers to display additional information above/below the player list.
    /// It is never sent by the Notchian server.
    PlayerListSetHeaderAndFooter {
        /// To remove the header, send a empty text component: `{"text":""}`
        header: Chat<'a>,
        /// To remove the footer, send a empty text component: `{"text":""}`
        footer: Chat<'a>,
    },

    // Todo add doc links
    /// Sent in response to Query Block NBT or Query Entity NBT.
    ///
    /// *Response to [ServerboundPacket::QueryBlockNbt] and [ServerboundPacket::QueryEntityNbt]*
    NbtQueryResponse {
        // Todo add doc link
        /// Can be compared to the one sent in the original query packet.
        query_id: VarInt,
        /// The NBT of the block or entity.
        /// May be a [NbtTag::Null] in which case no NBT is present.
        nbt_data: NbtTag,
    },

    // todo doc links
    /// Sent by the server when someone picks up an item lying on the ground.
    /// **Its sole purpose** appears to be the animation of the item flying towards you.
    /// **It doesn't destroy the entity in the client memory, and it doesn't add it to your inventory.**
    /// The server only checks for items to be picked up after each Player Position (and Player Position And Look) packet sent by the client.
    /// The collector entity can be any entity; it does not have to be a player.
    /// The collected entity also can be any entity, but the Notchian server only uses this for items, experience orbs, and the different varieties of arrows.
    CollectItem {
        collected_entity_id: VarInt,
        collector_entity_id: VarInt,
        /// Seems to be 1 for XP orbs, otherwise the number of items in the stack
        pickup_item_count: VarInt,
    },

    /// This packet is sent by the server when an entity moves more than 8 blocks.
    TeleportEntity {
        entity_id: VarInt,
        x: f64,
        y: f64,
        z: f64,
        /// New angle, not a delta
        yaw: Angle,
        /// New angle, not a delta
        pitch: Angle,
        on_ground: bool,
    },

    Advancements {
        /// Whether to reset/clear the current advancements
        reset: bool,
        advancement_mapping: Map<'a, Identifier<'a>, advancements::Advancement<'a>, VarInt>,
        /// The identifiers of the advancements that should be removed
        advancements_to_remove: Array<'a, Identifier<'a>, VarInt>,
        progress_mapping:
            Map<'a, Identifier<'a>, advancements::AdvancementProgress<'a>, VarInt>,
    },

    /// Sets [attributes](https://minecraft.fandom.com/wiki/Attribute) on the given entity
    EntityAttributes {
        entity_id: VarInt,
        /// [Attributes](entity::EntityAttribute) are a system of buffs/debuffs that are properties on mobs and players.
        /// [Attributes](entity::EntityAttribute) also have [Attributes](entity::EntityAttributeModifier) that adjust the strength of their effect.
        ///
        /// [More information](https://minecraft.fandom.com/wiki/Attribute)
        attributes: Map<'a, Identifier<'a>, entity::EntityAttribute<'a>, i32>,
    },

    EntityEffect {
        entity_id: VarInt,
        effect_id: effect::Effect,
        /// Notchian client displays effect level as `amplifier + 1`.
        /// For example, `Strength II` has an amplifier of 1.
        amplifier: i8,
        /// in thicks (1 thick = 50 ms)
        duration: VarInt,
        /// Bit field, see [the wiki](https://wiki.vg/Protocol#Entity_Properties).
        /// (should not be particularly useful)
        flags: u8,
    },

    DeclareRecipes {
        /// TODO
        data: RawBytes<'a>,
    },

    Tags {
        /// A map linking block tags to an array of corresponding IDs.
        block_tags: Map<'a, Identifier<'a>, Array<'a, VarInt, VarInt>, VarInt>,
        /// A map linking item tags to an array of corresponding IDs.
        item_tags: Map<'a, Identifier<'a>, Array<'a, VarInt, VarInt>, VarInt>,
        /// A map linking fluid tags to an array of corresponding IDs.
        fluid_tags: Map<'a, Identifier<'a>, Array<'a, VarInt, VarInt>, VarInt>,
        /// A map linking entity tags to an array of corresponding IDs.
        entity_tags: Map<'a, Identifier<'a>, Array<'a, VarInt, VarInt>, VarInt>,
    },
}
