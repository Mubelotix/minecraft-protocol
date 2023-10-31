
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_20() {
    let input = &[32, 11, 0, 0, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
