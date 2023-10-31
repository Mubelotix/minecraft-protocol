
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_14() {
    let input = &[20, 0, 0, 0, 0, 0, 152, 178, 218];
    ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
