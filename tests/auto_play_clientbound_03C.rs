
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_3c() {
    let input = &[60, 63, 1, 188, 156, 155, 209, 123, 5, 55, 70, 164, 19, 24, 123, 46, 142, 27, 81, 9, 77, 117, 98, 101, 108, 111, 116, 105, 120, 0, 0, 0, 1, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
