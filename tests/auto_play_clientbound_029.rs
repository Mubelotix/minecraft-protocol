
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_29() {
    let input = &[41, 0, 0, 9, 172, 0, 3, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 20, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 104, 101, 95, 110, 101, 116, 104, 101, 114, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 104, 101, 95, 101, 110, 100, 20, 10, 10, 0, 1, 0, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 151, 49, 254, 196, 135, 123, 8, 140, 0, 255, 0, 0, 1, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 255, 255, 169, 128, 0, 24, 128, 63, 0];
    ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
}
