
//! This test was automatically generated. Please run the proxy example to regenerate it.
//! 
//! ```
//! cargo run --example proxy
//! ```

use minecraft_protocol::{MinecraftPacketPart, packets::play_clientbound::ClientboundPacket};

#[test]
fn auto_play_clientbound_27() {
    let input = &[39, 7, 0, 64, 134, 68, 102, 102, 104, 0, 0, 64, 80, 92, 204, 204, 192, 0, 0, 64, 130, 19, 153, 153, 152, 0, 0, 61, 204, 204, 205, 0, 0, 0, 0, 61, 204, 204, 205, 62, 76, 204, 205, 0, 0, 0, 2];
    let packet = ClientboundPacket::deserialize_uncompressed_minecraft_packet(input).unwrap();

    match packet.serialize_minecraft_packet() {
        Ok(packet) => {
            assert_eq!(packet, input)
        }
        Err(e) => panic!("Failed to serialize packet: {:?}", e),
    };
}
