use super::*;

struct PlayerHandler {
    info: PlayerInfo,
    position: Position,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
    packet_sender: MpscSender<Vec<u8>>,
}

impl PlayerHandler {
    async fn send_packet<'a>(&mut self, packet: PlayClientbound<'a>) {
        let packet = packet.serialize_minecraft_packet().unwrap();
        self.packet_sender.send(packet).await.unwrap();
    }

    async fn on_server_message(&mut self, message: ServerMessage) {
        use ServerMessage::*;
        match message {
            Tick => {
                self.send_packet(PlayClientbound::BundleDelimiter).await;
            }
        }
    }

    async fn on_packet<'a>(&mut self, packet: PlayServerbound<'a>) {
        use PlayServerbound::*;
        match packet {
            SetPlayerPosition { x, y, z, on_ground } => {
                self.position.x = x;
                self.position.y = y;
                self.position.z = z;
                self.on_ground = on_ground;
                // TODO: make sure the movement is allowed
            },
            SetPlayerRotation { yaw, pitch, on_ground } => {
                self.yaw = yaw;
                self.pitch = pitch;
                self.on_ground = on_ground;
            }
            SetPlayerPositionAndRotation { x, y, z, yaw, pitch, on_ground } => {
                self.position.x = x;
                self.position.y = y;
                self.position.z = z;
                self.yaw = yaw;
                self.pitch = pitch;
                self.on_ground = on_ground;
                // TODO: make sure the movement is allowed
            },
            packet => warn!("Unsupported packet received: {packet:?}"),
        }
    }
}

pub async fn handle_player(stream: TcpStream, player_info: PlayerInfo, mut server_msg_rcvr: BroadcastReceiver<ServerMessage>) -> Result<(), ()> {
    let (packet_sender, mut packet_receiver) = mpsc_channel(100);
    
    let mut handler = PlayerHandler {
        info: player_info,
        position: Position { x: 0.0, y: 60.0, z: 0.0 },
        yaw: 0.0,
        pitch: 0.0,
        on_ground: false,
        packet_sender,
    };

    let (mut reader_stream, mut writer_stream) = stream.into_split();
    
    let mut receive_packet_fut = Box::pin(receive_packet_split(&mut reader_stream).fuse());
    let mut receive_clientbound_fut = Box::pin(packet_receiver.recv().fuse());
    let mut receive_server_message_fut = Box::pin(server_msg_rcvr.recv().fuse());
    loop {
        // Select the first event that happens
        enum Event {
            PacketServerbound(Vec<u8>),
            PacketClientbound(Option<Vec<u8>>),
            Message(Result<ServerMessage, BroadcastRecvError>),
        }
        let event = futures::select! {
            packet_serverbound = receive_packet_fut => Event::PacketServerbound(packet_serverbound),
            packet_clientbound = receive_clientbound_fut => Event::PacketClientbound(packet_clientbound),
            message = receive_server_message_fut => Event::Message(message),
        };
        match event {
            Event::PacketServerbound(packet) => {
                drop(receive_packet_fut);
                receive_packet_fut = Box::pin(receive_packet_split(&mut reader_stream).fuse());

                let packet = PlayServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
                handler.on_packet(packet).await;
            },
            Event::PacketClientbound(Some(packet)) => {
                drop(receive_clientbound_fut);
                receive_clientbound_fut = Box::pin(packet_receiver.recv().fuse());

                send_packet_raw_split(&mut writer_stream, packet.as_slice()).await;
            },
            Event::Message(Ok(message)) => {
                drop(receive_server_message_fut);
                receive_server_message_fut = Box::pin(server_msg_rcvr.recv().fuse());

                handler.on_server_message(message).await;
            },
            Event::Message(Err(recv_error)) => {
                error!("Failed to receive message: {recv_error:?}");
                return Err(());
            }
            Event::PacketClientbound(None) => {
                error!("Failed to receive clientbound packet");
                return Err(());
            }
        }
    }
}
