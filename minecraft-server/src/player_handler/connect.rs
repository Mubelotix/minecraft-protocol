use super::*;


#[instrument(skip_all)]
pub async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    server_msg_rcvr: BroadcastReceiver<ServerMessage>,
    world: &'static World,
) -> Result<(), ()> {
    // Receive handshake
    let packet = receive_packet(&mut stream).await?;
    let HandshakeServerbound::Hello { protocol_version, server_address, server_port, next_state } = HandshakeServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    match next_state {
        ConnectionState::Login => {
            let player_info = login(&mut stream, addr).await?;
            let (player_info, change_receiver) = handshake(&mut stream, player_info, world).await?;
            let uuid = player_info.uuid;
            let eid = Player::spawn_player(world, stream, player_info, server_msg_rcvr, change_receiver).await;
            Ok(())
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
