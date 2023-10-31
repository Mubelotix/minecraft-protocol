
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_6b() {
    let input = &[107, 141, 24, 64, 133, 58, 187, 88, 122, 112, 9, 192, 50, 148, 122, 225, 128, 0, 0, 64, 130, 116, 102, 208, 173, 152, 68, 66, 0, 0];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
