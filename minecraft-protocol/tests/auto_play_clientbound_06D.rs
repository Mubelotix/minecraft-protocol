
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_6d() {
    let input = &[109, 20, 5, 30, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 46, 97, 116, 116, 97, 99, 107, 95, 115, 112, 101, 101, 100, 64, 16, 0, 0, 0, 0, 0, 0, 0, 33, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 46, 97, 114, 109, 111, 114, 95, 116, 111, 117, 103, 104, 110, 101, 115, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 46, 109, 111, 118, 101, 109, 101, 110, 116, 95, 115, 112, 101, 101, 100, 63, 185, 153, 153, 160, 0, 0, 0, 0, 28, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 46, 109, 97, 120, 95, 104, 101, 97, 108, 116, 104, 64, 52, 0, 0, 0, 0, 0, 0, 0, 23, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 103, 101, 110, 101, 114, 105, 99, 46, 97, 114, 109, 111, 114, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            //reserialize let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            //reserialize assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
