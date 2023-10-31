
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_47() {
    let input = &[71, 29, 123, 34, 116, 101, 120, 116, 34, 58, 34, 65, 32, 77, 105, 110, 101, 99, 114, 97, 102, 116, 32, 83, 101, 114, 118, 101, 114, 34, 125, 0, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
