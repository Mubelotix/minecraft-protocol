
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_31() {
    let input = &[49, 1, 11, 34, 123, 34, 116, 114, 97, 110, 115, 108, 97, 116, 101, 34, 58, 34, 99, 111, 110, 116, 97, 105, 110, 101, 114, 46, 99, 114, 97, 102, 116, 105, 110, 103, 34, 125];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
