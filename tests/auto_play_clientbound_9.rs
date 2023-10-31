
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_9() {
    let input = &[9, 255, 255, 155, 128, 0, 19, 47, 206, 210, 164, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
