
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_2e() {
    let input = &[46, 255, 10, 224, 1, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
