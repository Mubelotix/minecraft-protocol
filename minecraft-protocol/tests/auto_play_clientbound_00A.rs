
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_a() {
    let input = &[10, 101, 3, 47, 136, 159, 200, 68, 101, 184, 212, 23, 111, 171, 57, 15, 227, 0, 130, 2, 123, 34, 105, 110, 115, 101, 114, 116, 105, 111, 110, 34, 58, 34, 101, 54, 49, 97, 100, 101, 55, 55, 45, 56, 99, 100, 99, 45, 52, 48, 54, 53, 45, 97, 49, 57, 99, 45, 55, 98, 102, 100, 102, 48, 101, 52, 100, 51, 50, 55, 34, 44, 34, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 34, 58, 123, 34, 97, 99, 116, 105, 111, 110, 34, 58, 34, 115, 104, 111, 119, 95, 101, 110, 116, 105, 116, 121, 34, 44, 34, 99, 111, 110, 116, 101, 110, 116, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 119, 105, 116, 104, 101, 114, 34, 44, 34, 105, 100, 34, 58, 34, 101, 54, 49, 97, 100, 101, 55, 55, 45, 56, 99, 100, 99, 45, 52, 48, 54, 53, 45, 97, 49, 57, 99, 45, 55, 98, 102, 100, 102, 48, 101, 52, 100, 51, 50, 55, 34, 44, 34, 110, 97, 109, 101, 34, 58, 123, 34, 116, 114, 97, 110, 115, 108, 97, 116, 101, 34, 58, 34, 101, 110, 116, 105, 116, 121, 46, 109, 105, 110, 101, 99, 114, 97, 102, 116, 46, 119, 105, 116, 104, 101, 114, 34, 125, 125, 125, 44, 34, 116, 114, 97, 110, 115, 108, 97, 116, 101, 34, 58, 34, 101, 110, 116, 105, 116, 121, 46, 109, 105, 110, 101, 99, 114, 97, 102, 116, 46, 119, 105, 116, 104, 101, 114, 34, 125, 63, 128, 0, 0, 5, 0, 1];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
