
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_20() {
    let input = &[32, 0, 255, 255, 161, 128, 0, 23, 112, 64, 1, 1];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
