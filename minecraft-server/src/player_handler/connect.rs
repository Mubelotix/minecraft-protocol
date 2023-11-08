use super::*;

pub async fn handle_connection(
    mut stream: TcpStream,
    addr: SocketAddr,
    server_msg_rcvr: BroadcastReceiver<ServerMessage>,
    world: Arc<World>,
) -> Result<(), ()> {
    // Receive handshake
    let packet = receive_packet(&mut stream).await;
    let HandshakeServerbound::Hello { protocol_version, server_address, server_port, next_state } = HandshakeServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    match next_state {
        ConnectionState::Login => {
            let player_info = login(&mut stream, addr).await?;
            let player_info = handshake(&mut stream, player_info).await?;
            let uuid = player_info.uuid;
            let r = handle_player(stream, player_info, server_msg_rcvr, Arc::clone(&world)).await;
            world.remove_loader(uuid).await;
            r
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
