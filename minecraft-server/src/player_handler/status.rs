use super::*;

#[cfg_attr(feature = "trace", instrument(skip_all))]
pub async fn status(stream: &mut TcpStream) -> Result<(), ()> {
    loop {
        let packet = receive_packet(stream).await?;
        match StatusServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap() {
            StatusServerbound::Request => {
                let response = StatusClientbound::Response {
                    json_response: include_str!("../raw/status_response.json")
                };
                send_packet(stream, response).await;    
                debug!("StatusResponse sent");                
            },
            StatusServerbound::Ping { payload } => {
                warn!("Ping received");
                let pong = StatusClientbound::Pong {
                    payload
                };
                send_packet(stream, pong).await;
                debug!("Pong sent");
                return Ok(());
            },
            _ => {
                debug!("Unexpected packet: {packet:?}");
            }
        };
    }
}
