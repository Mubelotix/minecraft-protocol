
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1() {
    let input = &[1, 166, 18, 46, 187, 149, 151, 91, 188, 74, 9, 168, 22, 104, 62, 7, 75, 212, 14, 118, 192, 119, 200, 0, 0, 0, 0, 0, 192, 70, 0, 0, 0, 0, 0, 0, 64, 123, 88, 0, 0, 0, 0, 0, 0, 249, 0, 0, 0, 0, 253, 141, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
