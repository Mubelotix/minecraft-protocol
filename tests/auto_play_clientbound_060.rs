
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_60() {
    let input = &[96, 0, 0, 0, 0, 0, 12, 30, 61, 0, 0, 0, 0, 0, 0, 19, 192];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
