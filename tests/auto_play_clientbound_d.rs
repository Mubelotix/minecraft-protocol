
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_d() {
    let input = &[13];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
