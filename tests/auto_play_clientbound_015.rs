
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_15() {
    let input = &[21, 0, 2, 0, 39, 1, 110, 1, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
