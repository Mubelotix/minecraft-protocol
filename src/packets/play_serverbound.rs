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
}
