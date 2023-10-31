use crate::nbt::NbtTag;

use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ClientboundPacket<'a> {
    PluginMessage {
        /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
        channel: &'a str,
        /// Any data. The length of this array must be inferred from the packet length.
        data: RawBytes<'a>,
    },
    
    Disconnect {
        reason: Chat<'a>,
    },

    /// Sent by the server to notify the client that the configuration process has finished. The client answers with its own Finish Configuration whenever it is ready to continue.
    FinishConfiguration {
        /// The reason why the player was disconnected
        reason: Chat<'a>,
    },

    /// The server will frequently send out a keep-alive, each containing a random ID. 
    /// The client must respond with the same payload ([see Serverbound Keep Alive](https://wiki.vg/Protocol#Serverbound_Keep_Alive_.28configuration.29)).
    ///  If the client does not respond to them for over 30 seconds, the server kicks the client. Vice versa, if the server does not send any keep-alives for 20 seconds, the client will disconnect and yields a "Timed out" exception.
    /// 
    /// The Notchian server uses a system-dependent time in milliseconds to generate the keep alive ID value.
    KeepAlive {
        keep_alive_id: i64,
    },

    /// Packet is not used by the Notchian server. When sent to the client, client responds with a Pong packet with the same id.
    Ping {
        id: i32,
    },

    /// Represents certain registries that are sent from the server and are applied on the client.
    RegistryData {
        registry_codec: NbtTag,
    }
}