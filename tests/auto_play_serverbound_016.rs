
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_16() {
    let input = &[22, 192, 120, 67, 87, 85, 251, 4, 53, 64, 80, 64, 0, 0, 0, 0, 0, 64, 125, 160, 222, 52, 11, 44, 88, 1];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
