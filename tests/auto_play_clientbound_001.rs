
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_1() {
    let input = &[1, 236, 23, 11, 155, 36, 194, 155, 30, 76, 18, 137, 233, 120, 123, 221, 13, 193, 64, 81, 64, 132, 235, 240, 157, 243, 3, 180, 64, 77, 90, 1, 187, 45, 209, 246, 64, 130, 105, 145, 110, 37, 116, 1, 0, 55, 55, 0, 0, 0, 255, 216, 0, 0];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
