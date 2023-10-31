
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1() {
    let input = &[1, 201, 10, 78, 73, 214, 36, 2, 99, 66, 206, 130, 48, 203, 171, 113, 237, 187, 175, 95, 192, 119, 116, 204, 204, 208, 0, 0, 192, 69, 115, 51, 51, 0, 0, 0, 64, 121, 123, 51, 51, 48, 0, 0, 0, 71, 68, 0, 0, 0, 3, 172, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
