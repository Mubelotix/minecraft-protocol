use super::*;

pub async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    server_msg_rcvr: BroadcastReceiver<ServerMessage>,
) -> Result<(), ()> {
    // Receive handshake
    let packet = receive_packet(&mut stream).await;
    let HandshakeServerbound::Hello { protocol_version, server_address, server_port, next_state } = HandshakeServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    match next_state {
        ConnectionState::Login => {
            let player_info = login(&mut stream, addr).await?;
            let player_info = handshake(&mut stream, player_info).await?;
            handle_player(stream, player_info, server_msg_rcvr).await
        },
        ConnectionState::Status => {
            status(&mut stream).await;
            Ok(())
        },
        _ => {
            error!("Unexpected next state: {next_state:?}");
            Err(())
        }
    }
}
