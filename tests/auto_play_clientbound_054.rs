
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_54() {
    let input = &[84, 172, 19, 9, 3, 65, 160, 0, 0, 16, 1, 2, 17, 0, 127, 255];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
