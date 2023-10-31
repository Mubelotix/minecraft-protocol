
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_23() {
    let input = &[35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 140, 156, 55, 0, 0, 0, 0, 65, 140, 156, 55, 0, 0, 0, 0, 0, 240, 134, 167, 14, 5, 15];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
