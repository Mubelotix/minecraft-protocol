
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_29() {
    let input = &[41, 0, 0, 0, 20, 0, 3, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 20, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 104, 101, 95, 110, 101, 116, 104, 101, 114, 17, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 116, 104, 101, 95, 101, 110, 100, 20, 10, 10, 0, 1, 0, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 116, 58, 202, 32, 182, 71, 111, 68, 1, 255, 0, 0, 1, 19, 109, 105, 110, 101, 99, 114, 97, 102, 116, 58, 111, 118, 101, 114, 119, 111, 114, 108, 100, 0, 0, 32, 128, 0, 25, 208, 52, 0];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            //reserialize let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            //reserialize assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
