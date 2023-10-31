
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_18() {
    let input = &[24, 195, 2, 204, 168, 65, 233, 255, 214, 0];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
