
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_2d() {
    let input = &[45, 213, 18, 1, 244, 0, 0, 0, 9, 197, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
