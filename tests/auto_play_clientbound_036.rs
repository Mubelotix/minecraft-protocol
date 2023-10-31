
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_36() {
    let input = &[54, 0, 61, 76, 204, 205, 61, 204, 204, 205];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
