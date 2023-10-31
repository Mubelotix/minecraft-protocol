
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_64() {
    let input = &[100, 245, 9, 6, 0, 0, 22, 75, 0, 0, 1, 248, 0, 0, 18, 65, 62, 204, 204, 205, 63, 140, 102, 141, 26, 180, 44, 217, 131, 111, 150, 28];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
