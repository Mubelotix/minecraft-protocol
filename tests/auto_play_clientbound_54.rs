
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_54() {
    let input = &[84, 237, 12, 9, 3, 65, 8, 0, 0, 17, 0, 127, 255];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
