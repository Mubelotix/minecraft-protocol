
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_13() {
    let input = &[19, 0, 1, 46, 0, 0, 0, 0, 0, 0, 1, 189, 6, 1, 10, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 4, 0, 0, 0, 1, 250, 4, 1, 0, 1, 161, 3, 3, 0, 1, 172, 6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 201, 6, 11, 0, 1, 145, 5, 1, 0, 1, 246, 5, 1, 10, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 4, 0, 1, 167, 6, 6, 0, 1, 237, 7, 1, 0, 1, 162, 6, 1, 10, 9, 0, 12, 69, 110, 99, 104, 97, 110, 116, 109, 101, 110, 116, 115, 10, 0, 0, 0, 1, 2, 0, 3, 108, 118, 108, 0, 5, 8, 0, 2, 105, 100, 0, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 115, 104, 97, 114, 112, 110, 101, 115, 115, 0, 3, 0, 6, 68, 97, 109, 97, 103, 101, 0, 0, 0, 0, 0, 1, 184, 3, 6, 0, 1, 153, 3, 1, 0, 1, 44, 1, 0, 1, 250, 4, 64, 0, 0, 0];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();
    
    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
