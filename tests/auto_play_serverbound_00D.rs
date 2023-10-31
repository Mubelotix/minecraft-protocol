
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_d() {
    let input = &[13, 0, 4, 0, 39, 0, 0, 1, 0, 39, 0, 1, 110, 3, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
