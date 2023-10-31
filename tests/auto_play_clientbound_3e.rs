
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_3e() {
    let input = &[62, 192, 119, 127, 76, 169, 128, 251, 80, 64, 80, 112, 198, 54, 117, 211, 101, 64, 119, 214, 87, 214, 60, 197, 227, 194, 255, 153, 78, 65, 221, 255, 215, 0, 1];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
