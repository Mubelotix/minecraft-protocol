use super::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ServerboundPacket<'a> {
    /// This causes the server to switch into the target state
    Hello {
        /// See [protocol version numbers](https://wiki.vg/Protocol_version_numbers) (currently 754 in Minecraft 1.16.5).
        protocol_version: VarInt,
        /// Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect.
        /// The Notchian server does not use this information.
        /// Note that SRV records are a complete redirect, e.g. if _minecraft._tcp.example.com points to mc.example.org, users connecting to example.com will provide mc.example.org as server address in addition to connecting to it.
        server_address: &'a str,
        /// Default is 25565.
        /// The Notchian server does not use this information.
        server_port: u16,
        next_state: ConnectionState,
    },
}
