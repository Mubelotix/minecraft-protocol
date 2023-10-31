
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_60() {
    let input = &[96, 0, 0, 0, 0, 0, 11, 124, 32, 0, 0, 0, 0, 0, 1, 116, 76];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
