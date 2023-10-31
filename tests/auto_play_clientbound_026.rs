
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_26() {
    let input = &[38, 0, 0, 7, 209, 255, 255, 159, 192, 0, 27, 160, 64, 0, 0, 42, 4, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
