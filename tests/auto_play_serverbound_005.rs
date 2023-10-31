
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_5() {
    let input = &[5, 4, 116, 101, 115, 116, 0, 0, 1, 139, 134, 2, 50, 72, 143, 72, 120, 119, 75, 31, 16, 4, 0, 0, 0, 0, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
