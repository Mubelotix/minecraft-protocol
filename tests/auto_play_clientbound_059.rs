
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_59() {
    let input = &[89, 65, 160, 0, 0, 20, 62, 76, 204, 208];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
