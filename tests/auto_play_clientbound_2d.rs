
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_2d() {
    let input = &[45, 205, 12, 255, 220, 0, 0, 9, 82, 0, 0, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
