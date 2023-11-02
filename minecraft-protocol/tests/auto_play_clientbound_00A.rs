
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_a() {
    let input = &[10, 51, 221, 3, 69, 59, 196, 67, 92, 134, 219, 94, 232, 56, 79, 240, 36, 0, 130, 2, 123, 34, 105, 110, 115, 101, 114, 116, 105, 111, 110, 34, 58, 34, 56, 100, 55, 55, 97, 100, 98, 101, 45, 102, 56, 54, 56, 45, 52, 101, 97, 51, 45, 56, 57, 48, 57, 45, 55, 50, 102, 53, 49, 51, 57, 53, 51, 48, 100, 98, 34, 44, 34, 104, 111, 118, 101, 114, 69, 118, 101, 110, 116, 34, 58, 123, 34, 97, 99, 116, 105, 111, 110, 34, 58, 34, 115, 104, 111, 119, 95, 101, 110, 116, 105, 116, 121, 34, 44, 34, 99, 111, 110, 116, 101, 110, 116, 115, 34, 58, 123, 34, 116, 121, 112, 101, 34, 58, 34, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 119, 105, 116, 104, 101, 114, 34, 44, 34, 105, 100, 34, 58, 34, 56, 100, 55, 55, 97, 100, 98, 101, 45, 102, 56, 54, 56, 45, 52, 101, 97, 51, 45, 56, 57, 48, 57, 45, 55, 50, 102, 53, 49, 51, 57, 53, 51, 48, 100, 98, 34, 44, 34, 110, 97, 109, 101, 34, 58, 123, 34, 116, 114, 97, 110, 115, 108, 97, 116, 101, 34, 58, 34, 101, 110, 116, 105, 116, 121, 46, 109, 105, 110, 101, 99, 114, 97, 102, 116, 46, 119, 105, 116, 104, 101, 114, 34, 125, 125, 125, 44, 34, 116, 114, 97, 110, 115, 108, 97, 116, 101, 34, 58, 34, 101, 110, 116, 105, 116, 121, 46, 109, 105, 110, 101, 99, 114, 97, 102, 116, 46, 119, 105, 116, 104, 101, 114, 34, 125, 63, 128, 0, 0, 5, 0, 1];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            //reserialize let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            //reserialize assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
