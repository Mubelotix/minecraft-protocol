
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_serverbound::ServerboundPacket};

#[test]
fn auto_play_serverbound_17() {
    let input = &[23, 64, 134, 62, 106, 79, 19, 95, 82, 64, 80, 64, 0, 0, 0, 0, 0, 64, 130, 25, 4, 162, 131, 6, 126, 67, 14, 166, 102, 65, 195, 153, 102, 0];
    let packet = ServerboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
