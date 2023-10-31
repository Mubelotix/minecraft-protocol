
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_64() {
    let input = &[100, 147, 2, 6, 255, 255, 244, 80, 0, 0, 2, 8, 0, 0, 14, 130, 62, 25, 153, 154, 63, 128, 0, 0, 3, 149, 6, 171, 119, 9, 62, 47];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
