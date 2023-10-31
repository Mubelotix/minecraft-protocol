
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_34() {
    let input = &[52, 0, 255, 255, 160, 128, 0, 23, 16, 63, 1, 63, 83, 174, 63, 63, 128, 0, 0, 63, 63, 57, 198, 0, 4];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
