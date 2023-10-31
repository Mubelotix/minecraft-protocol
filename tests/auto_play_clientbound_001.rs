
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1() {
    let input = &[1, 146, 45, 234, 6, 197, 73, 217, 127, 79, 232, 151, 167, 112, 71, 86, 187, 106, 141, 118, 64, 134, 172, 0, 0, 0, 0, 0, 64, 63, 0, 0, 0, 0, 0, 0, 64, 130, 68, 0, 0, 0, 0, 0, 0, 17, 17, 0, 0, 0, 253, 141, 0, 0];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
