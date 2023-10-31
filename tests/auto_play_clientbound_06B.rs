
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_6b() {
    let input = &[107, 152, 18, 192, 119, 232, 0, 0, 0, 0, 0, 192, 57, 0, 0, 0, 0, 0, 0, 64, 126, 232, 0, 0, 0, 0, 0, 183, 0, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
