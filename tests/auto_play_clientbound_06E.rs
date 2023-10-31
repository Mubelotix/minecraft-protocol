
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_6e() {
    let input = &[110, 172, 19, 15, 0, 255, 255, 255, 255, 15, 6, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
