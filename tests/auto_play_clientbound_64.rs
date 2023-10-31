
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_64() {
    let input = &[100, 141, 9, 5, 255, 255, 244, 36, 0, 0, 2, 0, 0, 0, 11, 187, 63, 128, 0, 0, 63, 128, 138, 78, 73, 89, 170, 187, 235, 254, 242, 160];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
