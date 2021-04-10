#[allow(unused_imports)]
use super::play_clientbound::ClientBoundPacket;

use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket {
    /// *Response to [ClientBoundPacket::PlayerPositionAndLook]*
    TeleportConfirm {
        /// The ID given in [ClientBoundPacket::PlayerPositionAndLook::teleport_id]
        teleport_id: VarInt,
    },
}
