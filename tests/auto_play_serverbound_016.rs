
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_16() {
    let input = &[22, 192, 113, 78, 128, 44, 40, 226, 33, 64, 82, 30, 157, 18, 187, 102, 162, 64, 125, 128, 75, 133, 48, 88, 174, 0];
    let packet_deserialized = ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            let _reserialized = ServerboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            assert!(matches!(ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
