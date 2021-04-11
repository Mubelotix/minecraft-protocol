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
        window_id: i8,
        /// Every action that is to be accepted has a unique number.
        /// This number is an incrementing integer (starting at 1) with separate counts for each window ID.
        action_number: i16,
        /// Whether the action was accepted
        accepted: bool,
    },
}
