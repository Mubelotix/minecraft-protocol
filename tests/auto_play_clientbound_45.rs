
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_45() {
    let input = &[69, 255, 255, 152, 0, 1, 64, 0, 3, 2, 234, 199, 252, 24, 235, 199, 251, 24];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
