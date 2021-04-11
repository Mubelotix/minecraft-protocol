#[allow(unused_imports)]
use super::play_clientbound::ClientboundPacket;

use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket<'a> {
    /// *Response to [ClientboundPacket::PlayerPositionAndLook]*
    TeleportConfirm {
        /// The ID given in [ClientboundPacket::PlayerPositionAndLook::teleport_id]
        teleport_id: VarInt,
    },

    /// *Request for [ClientboundPacket::NbtQueryResponse]*
    QueryBlockNbt {
        /// An incremental ID so that the client can verify that the response matches
        transaction_id: VarInt,
        /// The location of the block to check
        position: Position,
    },

    /// *Request for [ClientboundPacket::NbtQueryResponse]*
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
    /// *See also [ClientboundPacket::ChatMessage]*
    ChatMessage {
        /// The message may not be longer than 256 characters or else the server will kick the client.
        message: Chat<'a>,
    },

    /// *Request for [ClientboundPacket::Statistics]*
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

    /// *Request for [ClientboundPacket::TabComplete]*
    TabComplete {
        transaction_id: VarInt,
        /// All text behind the cursor without the `/` (e.g. to the left of the cursor in left-to-right languages like English).
        text: &'a str,
    },

    /// The server may reject client actions by sending [ClientboundPacket::WindowConfirmation] with the `accepted` field set to `false`.
    /// When this happens, the client must send this packet to apologize (as with movement), otherwise the server ignores any successive confirmations.
    ///
    /// *Response to [ClientboundPacket::WindowConfirmation]*
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
        /// The ID of the window sent by [ClientboundPacket::OpenWindow].
        window_id: u8,
        /// Meaning depends on window type; see [the wiki](https://wiki.vg/Protocol#Click_Window_Button)
        button_id: u8,
    },

    /// This packet is sent by the player when it clicks on a slot in a window.
    ///
    /// *Request for [ClientboundPacket::WindowConfirmation]*
    ClickWindowSlot {
        /// The ID of the window which was clicked. 0 for player inventory.
        window_id: u8,
        /// The clicked slot number, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        slot: i16,
        /// The button used in the click, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        button: u8,
        /// A unique number for the action, implemented by Notchian as a counter, starting at 1 (different counter for every window ID). Used by the server to send back a [ClientboundPacket::WindowConfirmation].
        action_id: i16,
        /// Inventory operation mode, see [the wiki](https://wiki.vg/Protocol#Click_Window)
        mode: VarInt,
        /// The clicked slot. Has to be empty (item ID = -1) for drop mode. (TODO: check this)
        clicked_item: crate::slots::Slot<'a>,
    },

    /// This packet is sent by the client when closing a window.
    /// Notchian clients send a Close Window packet with `window_id` = 0 to close their inventory even though there is never an [ClientboundPacket::OpenWindow] packet for the inventory.
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
    /// *See also [ClientboundPacket::PluginMessage]*
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

    /// *Response to [ClientboundPacket::KeepAlive]*
    KeepAlive {
        /// The id sent in the [ClientboundPacket::KeepAlive] packet
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
    /// If the player is moving too fast, it will be logged that "<player> moved too quickly! " followed by the change in x, y, and z, and the player will be [teleported](ClientboundPacket::TeleportEntity) back to their current (before this packet) serverside position.
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

    /// Used to visually update whether boat paddles are turning.
    /// The server will update the [Boat entity metadata](https://wiki.vg/Entities#Boat) to match the values here.
    SteerBoat {
        /// Left paddle turning is set to true when the right button or forward button is held.
        left_paddle_turnin: bool,
        /// Right paddle turning is set to true when the left button or forward button is held.
        right_paddle_turnin: bool,
    },

    /// Used to swap out an empty space on the hotbar with the item in the given inventory slot.
    /// The Notchain client uses this for pick block functionality (middle click) to retrieve items from the inventory.
    ///
    /// The server will first search the player's hotbar for an empty slot, starting from the current slot and looping around to the slot before it.
    /// If there are no empty slots, it will start a second search from the current slot and find the first slot that does not contain an enchanted item.
    /// If there still are no slots that meet that criteria, then the server will use the currently selected slot.
    ///
    /// After finding the appropriate slot, the server swaps the items and then send 3 packets:
    /// - [ClientboundPacket::SetSlot], with window ID set to -2 and slot set to the newly chosen slot and the item set to the item that is now in that slot (which was previously at the slot the client requested)
    /// - [ClientboundPacket::SetSlot], with window ID set to -2 and slot set to the slot the player requested, with the item that is now in that slot and was previously on the hotbar slot
    /// - [ClientboundPacket::HeldItemChange], with the slot set to the newly chosen slot.
    PickItem {
        /// See [inventory](https://wiki.vg/Inventory)
        slot_to_use: VarInt,
    },

    /// This packet is sent when a player clicks a recipe in the crafting book that is craftable (white border).
    CraftRecipeRequest {
        window_id: u8,
        recipe_id: Identifier<'a>,
        make_all: bool,
    },

    /// The vanilla client sends this packet when the player starts/stops flying with the `flags` field changed accordingly.
    PlayerAbilities {
        /// Bit mask. 0x02: is flying.
        flags: u8,
    },

    /// Sent when the player mines a block.
    /// A Notchian server only accepts digging packets with coordinates within a 6-unit radius between the center of the block and 1.5 units from the player's feet (not their eyes).
    DigBlock {
        /// The action the player is taking against the block
        status: crate::blocks::DiggingState,
        /// Block position
        location: Position,
        /// The face being hit
        face: crate::blocks::BlockFace,
    },

    /// Sent by the client to indicate that it has performed certain actions: sneaking (crouching), sprinting, exiting a bed, jumping with a horse, and opening a horse's inventory while riding it.
    EntityAction {
        player_id: VarInt,
        action_id: crate::entity::PlayerAction,
        /// Only used by the [“start jump with horse” action](crate::entity::PlayerAction::StartJumpWithHorse), in which case it ranges from 0 to 100. In all other cases it is 0.
        jump_boost: bool,
    },

    SteerVehicle {
        /// Movement to the left, can be negative to move to the right.
        to_the_left: f32,
        /// Movement forward, can be negative to move backward.
        forward: f32,
        /// Bit mask. 0x1: jump, 0x2: unmount.
        flags: u8,
    },

    /// Replaces Recipe Book Data, type 1.
    SetRecipeBookState {
        book: crate::recipes::RecipeBook,
        is_open: bool,
        is_filter_active: bool,
    },

    /// Replaces Recipe Book Data, type 0.
    SetDisplayedRecipe { recipe_id: Identifier<'a> },

    /// Sent as a player is renaming an item in an anvil (each keypress in the anvil UI sends a new Name Item packet).
    /// If the new name is empty, then the item loses its custom name (this is different from setting the custom name to the normal name of the item).
    NameItem {
        /// The item name may be no longer than 35 characters long, and if it is longer than that, then the rename is silently ignored.
        new_name: &'a str,
    },
}
