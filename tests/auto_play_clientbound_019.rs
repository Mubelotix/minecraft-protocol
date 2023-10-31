
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_19() {
    let input = &[25, 137, 1, 31, 173, 19, 173, 19, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
