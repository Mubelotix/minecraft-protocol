
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_4() {
    let input = &[4, 14, 116, 105, 109, 101, 32, 115, 101, 116, 32, 110, 105, 103, 104, 116, 0, 0, 1, 139, 134, 2, 46, 165, 224, 111, 128, 99, 13, 123, 139, 212, 0, 0, 0, 0, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
