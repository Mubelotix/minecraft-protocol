
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_2() {
    let input = &[2, 247, 20, 192, 119, 92, 100, 125, 116, 76, 209, 64, 80, 0, 0, 0, 0, 0, 0, 64, 124, 50, 149, 33, 43, 228, 31, 0, 3];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
