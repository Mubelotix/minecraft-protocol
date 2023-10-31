
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_58() {
    let input = &[88, 62, 146, 73, 37, 0, 2];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
