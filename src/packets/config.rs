use crate::{nbt::NbtTag, components::{chat, players, resource_pack}};

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
    FinishConfiguration,
    
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
    },

    RessourcePakck {
        /// The URL to the resource pack.
        url: &'a str,
        /// A 40 character hexadecimal and lowercase SHA-1 hash of the resource pack file.
        /// If it's not a 40 character hexadecimal string, the client will not use it for hash verification and likely waste bandwidth — but it will still treat it as a unique id
        hash: &'a str,
        /// The notchian client will be forced to use the resource pack from the server. If they decline they will be kicked from the server.
        forced: bool,
        /// This is shown in the prompt making the client accept or decline the resource pack.
        prompt_message: Option<Chat<'a>>
    },

    /// Used to enable and disable features, generally experimental ones, on the client.
    FeatureFlags {
        features: Array<'a, Identifier<'a>, VarInt>
    },

    UpdateTags {
        tags: Map<'a, Identifier<'a>, Map<'a, Identifier<'a>, Array<'a, VarInt, VarInt>, VarInt>, VarInt>
    }
}

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket<'a> {
    /// Sent when the player connects, or when settings are changed.
    ClientInformations {
        /// e.g. "en_US"
        locale: &'a str,
        /// Client-side render distance, in chunks.
        render_distance: i8,
        chat_mode: chat::ChatMode,
        /// “Colors” multiplayer setting. Can the chat be colored?
        chat_colors: bool,
        /// Bit mask see [here](https://wiki.vg/Protocol#Client_Information_.28configuration.29)
        displayed_skin_parts: u8,
        main_hand: players::MainHand,
        /// Enables filtering of text on signs and written book titles. Currently always false (i.e. the filtering is disabled)
        enable_text_filtering: bool,
        /// Servers usually list online players, this option should let you not show up in that list.
        allow_server_listing: bool,
    },

    /// Mods and plugins can use this to send their data. Minecraft itself uses some plugin channels. These internal channels are in the `minecraft` namespace.
    PluginMessage {
        /// Name of the [plugin channel](https://wiki.vg/Plugin_channel) used to send the data.
        channel: &'a str,
        /// Any data, depending on the channel. `minecraft:` channels are documented [here](https://wiki.vg/Plugin_channel). The length of this array must be inferred from the packet length.
        data: RawBytes<'a>,
    },

    /// Sent by the client to notify the client that the configuration process has finished. It is sent in response to the server's [Finish Configuration](https://wiki.vg/Protocol#Finish_Configuration).
    FinishConfiguration,

    /// The server will frequently send out a keep-alive (see [Clientbound Keep Alive](https://wiki.vg/Protocol#Clientbound_Keep_Alive_.28configuration.29)), each containing a random ID. The client must respond with the same packet.
    KeepAlive {
        id: i64,
    },

    /// Response to the clientbound packet (Ping) with the same id.
    Pong {
        id: i32,
    },
    
    ResourcePackResponse {
        result: resource_pack::ResourcePackStatus,
    },
}
