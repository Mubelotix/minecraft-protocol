use crate::*;

/// The possible packets are different for each state.
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ConnectionState {
    /// The possible packets are listed in [handshake].
    HandShake,
    /// The possible packets are listed in [status].
    Status,
    /// The possible packets are listed in [login].
    Login,
    /// The possible packets are listed in [play_clientbound] and [play_serverbound].
    Play,
}

/// Information given by an authority (like Mojang) to check user authentification.
#[derive(Debug, MinecraftPacketPart)]
pub struct LoginSignature<'a> {
    /// When the key data will expire.
    pub expiration_ts: i64,
    /// The public key the client received from Mojang.
    pub public_key: Array<'a, u8, VarInt>,
    /// The public key signature the client received from Mojang.
    pub signature: Array<'a, u8, VarInt>,
}
