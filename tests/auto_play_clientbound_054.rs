
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_54() {
    let input = &[84, 235, 23, 9, 3, 65, 30, 31, 100, 16, 1, 70, 17, 0, 127, 255];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
