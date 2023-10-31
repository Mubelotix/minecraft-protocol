
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_67() {
    let input = &[103, 162, 1, 123, 34, 101, 120, 116, 114, 97, 34, 58, 91, 123, 34, 98, 111, 108, 100, 34, 58, 102, 97, 108, 115, 101, 44, 34, 105, 116, 97, 108, 105, 99, 34, 58, 102, 97, 108, 115, 101, 44, 34, 117, 110, 100, 101, 114, 108, 105, 110, 101, 100, 34, 58, 102, 97, 108, 115, 101, 44, 34, 115, 116, 114, 105, 107, 101, 116, 104, 114, 111, 117, 103, 104, 34, 58, 102, 97, 108, 115, 101, 44, 34, 111, 98, 102, 117, 115, 99, 97, 116, 101, 100, 34, 58, 102, 97, 108, 115, 101, 44, 34, 99, 111, 108, 111, 114, 34, 58, 34, 121, 101, 108, 108, 111, 119, 34, 44, 34, 116, 101, 120, 116, 34, 58, 34, 65, 101, 108, 111, 114, 105, 117, 115, 32, 106, 111, 105, 110, 101, 100, 32, 116, 104, 101, 32, 103, 97, 109, 101, 34, 125, 93, 44, 34, 116, 101, 120, 116, 34, 58, 34, 34, 125, 0];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
