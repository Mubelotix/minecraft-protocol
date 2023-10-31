
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_9() {
    let input = &[9, 255, 255, 157, 192, 0, 24, 224, 63, 10];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
