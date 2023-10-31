
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1() {
    let input = &[1, 16, 132, 189, 148, 153, 195, 221, 77, 59, 173, 245, 108, 213, 148, 63, 37, 5, 118, 192, 111, 49, 90, 208, 18, 24, 87, 192, 32, 0, 0, 0, 0, 0, 0, 64, 125, 4, 12, 147, 75, 225, 24, 0, 248, 87, 0, 0, 0, 253, 141, 0, 0];
    let packet_deserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet_deserialized.serialize_minecraft_packet() {
        Ok(packet) => {
            //reserialize let _reserialized = ClientboundPacket::deserialize_uncompressed_minecraft_packet(&packet).unwrap();
            //reserialize assert!(matches!(ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap(), _reserialized));
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
