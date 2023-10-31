
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_27() {
    let input = &[39, 7, 0, 192, 119, 92, 100, 125, 116, 76, 209, 64, 80, 22, 102, 102, 96, 0, 0, 64, 124, 50, 149, 33, 43, 228, 31, 61, 204, 204, 205, 0, 0, 0, 0, 61, 204, 204, 205, 62, 76, 204, 205, 0, 0, 0, 2];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
