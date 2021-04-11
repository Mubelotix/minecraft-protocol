#[allow(unused_imports)]
use super::play_clientbound::ClientBoundPacket;

use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket<'a> {
    /// *Response to [ClientBoundPacket::PlayerPositionAndLook]*
    TeleportConfirm {
        /// The ID given in [ClientBoundPacket::PlayerPositionAndLook::teleport_id]
        teleport_id: VarInt,
    },

    /// *Request for [ClientBoundPacket::NbtQueryResponse]*
    QueryBlockNbt {
        /// An incremental ID so that the client can verify that the response matches
        transaction_id: VarInt,
        /// The location of the block to check
        position: Position,
    },

    /// *Request for [ClientBoundPacket::NbtQueryResponse]*
    QueryEntityNbt {
        /// An incremental ID so that the client can verify that the response matches
        transaction_id: VarInt,
        entity_id: VarInt,
    },

    /// Appears to only be used on singleplayer; the difficulty buttons are still disabled in multiplayer.
    SetDifficulty {
        new_difficulty: crate::difficulty::Difficulty,
    },

    /// Used to send a chat message to the server.
    ///
    /// If the message starts with a /, the server will attempt to interpret it as a command.
    /// Otherwise, the server will broadcast the same chat message to all players on the server (including the player that sent the message), prepended with player's name.
    ///
    /// *See also [ClientBoundPacket::ChatMessage]*
    ChatMessage {
        /// The message may not be longer than 256 characters or else the server will kick the client.
        message: Chat<'a>,
    },

    /// *Request for [ClientBoundPacket::Statistics]*
    ClientStatus {
        action: crate::game_state::ClientStatus,
    },

    /// Sent when the player connects, or when settings are changed
    ClientSettings {
        /// e.g. `en_GB`
        locale: &'a str,
        /// Client-side render distance, in chunks
        render_distance: u8,
        chat_mode: crate::chat::ChatMode,
        /// “Colors” multiplayer setting
        chat_colors_enabled: bool,
        /// Bit mask, see [the wiki](https://wiki.vg/Protocol#Client_Settings)
        displayed_skin_parts: u8,
        main_hand: crate::slots::MainHand,
    },

    /// *Request for [ClientBoundPacket::TabComplete]*
    TabComplete {
        transaction_id: VarInt,
        /// All text behind the cursor without the `/` (e.g. to the left of the cursor in left-to-right languages like English).
        text: &'a str,
    },

    /// The server may reject client actions by sending [ClientBoundPacket::WindowConfirmation] with the `accepted` field set to `false`.
    /// When this happens, the client must send this packet to apologize (as with movement), otherwise the server ignores any successive confirmations.
    ///
    /// *Response to [ClientBoundPacket::WindowConfirmation]*
    WindowConfirmation {
        /// The ID of the window that the action occurred in
        window_id: u8,
        /// Every action that is to be accepted has a unique number.
        /// This number is an incrementing integer (starting at 1) with separate counts for each window ID.
        action_number: i16,
        /// Whether the action was accepted
        accepted: bool,
    },

    /// Used when clicking on window buttons
    ClickWindowButton {
        /// The ID of the window sent by [ClientBoundPacket::OpenWindow].
        window_id: u8,
        /// Meaning depends on window type; see [the wiki](https://wiki.vg/Protocol#Click_Window_Button)
        button_id: u8,
    },

    /// This packet is sent by the player when it clicks on a slot in a window.
    ///
    /// *Request for [ClientBoundPacket::WindowConfirmation]*
    ClickWindowSlot {
        /// The ID of the window which was clicked. 0 for player inventory.
        window_id: u8,
        /// The clicked slot number, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        slot: i16,
        /// The button used in the click, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        button: u8,
        /// A unique number for the action, implemented by Notchian as a counter, starting at 1 (different counter for every window ID). Used by the server to send back a [ClientBoundPacket::WindowConfirmation].
        action_id: i16,
        /// Inventory operation mode, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        mode: VarInt,
        /// The clicked slot. Has to be empty (item ID = -1) for drop mode. (TODO: check this)
        clicked_item: crate::slots::Slot<'a>,
    },

    /// This packet is sent by the client when closing a window.
    /// Notchian clients send a Close Window packet with `window_id` = 0 to close their inventory even though there is never an [ClientBoundPacket::OpenWindow] packet for the inventory.
    CloseWindow {
        /// The ID of the window that was closed. 0 for player inventory.
        window_id: u8,
    },

    /// Mods and plugins can use this to send their data.
    /// Minecraft itself uses some [plugin channels](https://wiki.vg/Plugin_channel).
    /// These internal channels are in the `minecraft` namespace.
    ///
    /// [More documentation](http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/)
    ///
    /// *See also [ClientBoundPacket::PluginMessage]*
    PluginMessage {
        /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
        identifier: Identifier<'a>,
        /// Any data, depending on the channel.
        /// `minecraft:` channels are documented [here](https://wiki.vg/Plugin_channel).
        data: RawBytes<'a>,
    },

    EditBook {
        /// See [the wiki](https://wiki.vg/Protocol#Edit_Book) for information about the NBT data structure of this slot.
        new_book: crate::slots::Slot<'a>,
        /// `true` if the player is signing the book; `false` if the player is saving a draft.
        is_signing: bool,
        hand: crate::slots::Hand,
    },

    /// This packet is sent from the client to the server when the client attacks or right-clicks another entity (a player, minecart, etc).
    /// A Notchian server only accepts this packet if the entity being attacked/used is visible without obstruction and within a 4-unit radius of the player's position.
    /// Note that middle-click in creative mode is interpreted by the client and sent as a [ServerboundPacket::CreativeInventoryAction] packet instead.
    InteractEntity {
        entity_id: VarInt,
        interaction_type: crate::entity::EntityInteraction,
        sneaking: bool,
    },

    /// Sent when Generate is pressed on the [Jigsaw Block](http://minecraft.gamepedia.com/Jigsaw_Block) interface.
    GenerateStructure {
        /// Block entity location
        location: Position,
        /// Value of the levels slider/max depth to generate
        levels: VarInt,
        keep_jigsaws: bool,
    },

    /// *Response to [ClientBoundPacket::KeepAlive]*
    KeepAlive {
        /// The id sent in the [ClientBoundPacket::KeepAlive] packet
        keep_alive_id: u64,
    },

    /// Appears to only be used on singleplayer; the difficulty buttons are still disabled in multiplayer.
    LockDifficulty { locked: bool },

    /// Updates the player's position on the server.
    ///
    /// Checking for moving too fast is achieved like this:
    /// - Each server tick, the player's current position is stored
    /// - When a player moves, the changes in `x`, `y`, and `z` coordinates are compared with the positions from the previous tick (`Δx`, `Δy`, `Δz`)
    /// - *Total movement distance* squared is computed as `Δx² + Δy² + Δz²`
    /// - The *expected movement distance* squared is computed as `velocityX² + veloctyY² + velocityZ²`
    /// - If the *total movement distance* squared value minus the *expected movement distance* squared value is more than 100 (300 if the player is using an elytra), they are moving too fast.
    /// If the player is moving too fast, it will be logged that "<player> moved too quickly! " followed by the change in x, y, and z, and the player will be [teleported](ClientBoundPacket::TeleportEntity) back to their current (before this packet) serverside position.
    PlayerPosition {
        x: f64,
        /// The feet position (`feet_y = head_y - 1.62`)
        y: f64,
        z: f64,
        /// `true` if the client is on the ground, `false` otherwise
        on_ground: bool,
    },

    /// A combination of [ServerboundPacket::PlayerRotation] and [ServerboundPacket::PlayerPosition]
    PlayerPositionAndRotation {
        x: f64,
        /// The feet position (`feet_y = head_y - 1.62`)
        y: f64,
        z: f64,
        /// Absolute rotation on the X Axis, in degrees.
        /// [Learn more about yaw and pitch](https://wiki.vg/Protocol#Player_Rotation)
        yaw: f32,
        /// Absolute rotation on the Y Axis, in degrees
        /// [Learn more about yaw and pitch](https://wiki.vg/Protocol#Player_Rotation)
        pitch: f32,
        /// `true` if the client is on the ground, `false` otherwise
        on_ground: bool,
    },

    /// Updates the direction the player is looking in
    PlayerRotation {
        /// Absolute rotation on the X Axis, in degrees.
        /// [Learn more about yaw and pitch](https://wiki.vg/Protocol#Player_Rotation)
        yaw: f32,
        /// Absolute rotation on the Y Axis, in degrees, where 0 is looking straight ahead, -90 is looking straight up, and 90 is looking straight down.
        /// [Learn more about yaw and pitch](https://wiki.vg/Protocol#Player_Rotation)
        pitch: f32,
        /// `true` if the client is on the ground, `false` otherwise
        on_ground: bool,
    },

    /// This packet is used to indicate whether the player is on ground (walking/swimming), or airborne (jumping/falling).
    ///
    /// Vanilla clients will send Player Position once every 20 ticks even for a stationary player.
    ///
    /// When dropping from sufficient height, fall damage is applied when this state goes from false to true.
    /// The amount of damage applied is based on the point where it last changed from true to false.
    /// Note that there are several movement related packets containing this state.
    PlayerFulcrum {
        /// `true` if the client is on the ground, `false` otherwise
        on_ground: bool,
    },
}
