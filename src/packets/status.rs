//! For general information about pinging servers, see [the wiki](https://wiki.vg/Server_List_Ping).

use crate::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ClientboundPacket<'a> {
    /// *Response to [ServerboundPacket::Request]*
    Response {
        /// See [Server List Ping](https://wiki.vg/Server_List_Ping#Response); as with all strings this is prefixed by its length as a VarInt.
        json_response: &'a str,
    },

    /// *Response to [ServerboundPacket::Ping]*
    Pong {
        /// Should be the same as sent by the client
        payload: i64,
    },
}

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket {
    /// *Request for [ClientboundPacket::Response]*
    Request,

    /// *Request for [ClientboundPacket::Pong]*
    Ping {
        /// May be any number.  
        /// Notchian clients use a system-dependent time value which is counted in milliseconds.
        payload: i64,
    },
}
