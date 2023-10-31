
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_18() {
    let input = &[24, 194, 240, 76, 235, 65, 249, 153, 182, 1];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
