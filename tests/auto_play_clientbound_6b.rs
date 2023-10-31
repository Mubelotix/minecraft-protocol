
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_6b() {
    let input = &[107, 253, 10, 192, 119, 240, 5, 206, 220, 49, 182, 64, 80, 0, 0, 0, 0, 0, 0, 64, 120, 39, 215, 202, 68, 131, 66, 128, 0, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
