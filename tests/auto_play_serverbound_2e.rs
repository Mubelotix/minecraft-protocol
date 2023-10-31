
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_2e() {
    let input = &[46, 0, 36, 1, 154, 1, 1, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
