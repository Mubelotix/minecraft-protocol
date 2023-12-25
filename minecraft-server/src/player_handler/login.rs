use super::*;

pub struct LoggedInPlayerInfo {
    pub(super) addr: SocketAddr,
    pub(super) username: String,
    pub(super) uuid: u128,
}

#[cfg_attr(feature = "trace", instrument(skip_all))]
pub async fn login(stream: &mut TcpStream, addr: SocketAddr) -> Result<LoggedInPlayerInfo, ()> {
    // Receive login start
    let packet = receive_packet(stream).await?;
    let packet = LoginServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    let LoginServerbound::LoginStart{ username, player_uuid } = packet else {
        error!("Expected LoginStart packet, got: {packet:?}");
        return Err(());
    };
    debug!("LoginStart: {username}");

    // TODO encryption

    // TODO compression

    // Send login success
    let login_success = LoginClientbound::LoginSuccess {
        uuid: player_uuid,
        username,
        properties: Array::default(),
    };
    send_packet(stream, login_success).await;
    debug!("LoginSuccess sent");

    // Receive login acknowledged
    let packet = receive_packet(stream).await?;
    let packet = LoginServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    let LoginServerbound::LoginAcknowledged = packet else {
        error!("Expected LoginAcknowledged packet, got: {packet:?}");
        return Err(());
    };
    debug!("LoginAcknowledged received");

    // Ignore encryption response if any
    let packet = receive_packet(stream).await?;
    if let Ok(LoginServerbound::EncryptionResponse { .. }) = LoginServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()) {
        // Ignore for now (TODO)
        //packet = receive_packet(stream).await?;
    }
    debug!("EncryptionResponse ignored");

    Ok(LoggedInPlayerInfo {
        addr,
        username: username.to_owned(),
        uuid: player_uuid,
    })
}
