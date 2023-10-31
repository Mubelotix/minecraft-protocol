
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_26() {
    let input = &[38, 0, 0, 7, 209, 255, 255, 161, 192, 0, 23, 32, 64, 0, 0, 7, 213, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
