
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1f() {
    let input = &[31, 0, 0, 0, 32, 255, 255, 255, 226];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
