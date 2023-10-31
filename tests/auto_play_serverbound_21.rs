
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_21() {
    let input = &[33, 237, 12, 0, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}