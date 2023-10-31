
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_17() {
    let input = &[23, 192, 113, 135, 164, 214, 221, 178, 146, 64, 81, 130, 2, 71, 54, 11, 25, 64, 125, 125, 228, 168, 24, 15, 226, 194, 226, 128, 63, 66, 110, 204, 151, 0];
    let packet_deserialized = ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            let _reserialized = ServerboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            assert!(matches!(ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
