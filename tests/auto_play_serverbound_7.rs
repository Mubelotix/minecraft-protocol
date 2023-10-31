
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_7() {
    let input = &[7, 64, 36, 98, 131];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}