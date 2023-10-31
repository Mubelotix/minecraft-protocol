
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_16() {
    let input = &[22, 192, 119, 122, 164, 154, 100, 165, 236, 64, 80, 112, 198, 54, 117, 211, 101, 64, 119, 186, 184, 207, 14, 17, 5, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
