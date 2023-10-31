
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_3e() {
    let input = &[62, 192, 120, 73, 220, 74, 22, 145, 128, 64, 80, 64, 0, 0, 0, 0, 0, 64, 125, 163, 19, 232, 50, 29, 211, 194, 241, 51, 82, 65, 249, 153, 182, 0, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
