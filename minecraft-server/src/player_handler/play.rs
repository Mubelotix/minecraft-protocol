use super::*;

struct PlayerHandler {
    world: Arc<World>,
    game_mode: Gamemode,
    info: PlayerInfo,
    position: Position,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
    packet_sender: MpscSender<Vec<u8>>,

    render_distance: i32,
    loaded_chunks: HashSet<ChunkColumnPosition>,
    center_chunk: ChunkPosition,
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

    async fn on_block_change(&mut self, position: BlockPosition, block: BlockWithState) {
        self.send_packet(PlayClientbound::BlockUpdate {
            location: position.into(),
            block_state: block,
        }).await;
    }

    async fn on_move(&mut self) {
        let new_center_chunk = self.position.chunk();

        // Tell the client which chunk he is in
        if new_center_chunk == self.center_chunk { return };
        self.send_packet(PlayClientbound::SetCenterChunk { chunk_x: VarInt(new_center_chunk.cx), chunk_z: VarInt(new_center_chunk.cz) }).await;

        // Find out which chunks should be loaded
        if new_center_chunk.chunk_column() == self.center_chunk.chunk_column() {
            self.center_chunk = new_center_chunk;
            return;
        };
        let mut loaded_chunks_after = HashSet::new();
        for cx in (new_center_chunk.cx - self.render_distance)..=(new_center_chunk.cx + self.render_distance) {
            for cz in (new_center_chunk.cz - self.render_distance)..=(new_center_chunk.cz + self.render_distance) {
                let dist = (((cx - new_center_chunk.cx).pow(2) + (cz - new_center_chunk.cz).pow(2)) as f32).sqrt();
                if dist > self.render_distance as f32 { continue };
                loaded_chunks_after.insert(ChunkColumnPosition { cx, cz });
            }
        }

        // Select chunks to load (max 50) and unload
        if loaded_chunks_after == self.loaded_chunks { return };
        let mut newly_loaded_chunks: Vec<_> = loaded_chunks_after.difference(&self.loaded_chunks).cloned().collect();
        let unloaded_chunks: Vec<_> = self.loaded_chunks.difference(&loaded_chunks_after).cloned().collect();
        for skipped in newly_loaded_chunks.iter().skip(50) {
            loaded_chunks_after.remove(skipped);
        }
        newly_loaded_chunks.truncate(50);

        // Tell the world about the changes
        self.world.update_loaded_chunks(self.info.uuid, loaded_chunks_after.clone()).await;

        // Send the chunks to the client
        for newly_loaded_chunk in newly_loaded_chunks {
            let mut column = Vec::new();
            let heightmaps = self.world.get_network_heightmap(newly_loaded_chunk.clone()).await.unwrap_or_else(|| {
                error!("Chunk not loaded: {newly_loaded_chunk:?}");
                NbtTag::Compound(HashMap::new()) // TODO hard error
            });

            for cy in -4..20 {
                let chunk = self.world.get_network_chunk(newly_loaded_chunk.chunk(cy)).await.unwrap_or_else(|| {
                    error!("Chunk not loaded: {newly_loaded_chunk:?}");
                    NetworkChunk { // TODO hard error
                        block_count: 0,
                        blocks: PalettedData::Single { value: 0 },
                        biomes: PalettedData::Single { value: 4 },
                    }
                });
                column.push(chunk);
            }
            let serialized: Vec<u8> = NetworkChunk::into_data(column).unwrap();
            let chunk_data = PlayClientbound::ChunkData {
                value: ChunkData {
                    chunk_x: newly_loaded_chunk.cx,
                    chunk_z: newly_loaded_chunk.cz,
                    heightmaps,
                    data: Array::from(serialized.clone()),
                    block_entities: Array::default(),
                    sky_light_mask: Array::default(),
                    block_light_mask: Array::default(),
                    empty_sky_light_mask: Array::default(),
                    empty_block_light_mask: Array::default(),
                    sky_light: Array::default(),
                    block_light: Array::default(),
                }
            };
            self.send_packet(chunk_data).await;
        }

        // Tell the client to unload chunks
        for unloaded_chunk in unloaded_chunks {
            self.send_packet(PlayClientbound::UnloadChunk {
                chunk_x: unloaded_chunk.cx,
                chunk_z: unloaded_chunk.cz,
            }).await;
        }

        self.loaded_chunks = loaded_chunks_after;
    }

    async fn on_packet<'a>(&mut self, packet: PlayServerbound<'a>) {
        use PlayServerbound::*;
        match packet {
            SetPlayerPosition { x, y, z, on_ground } => {
                self.position.x = x;
                self.position.y = y;
                self.position.z = z;
                self.on_ground = on_ground;
                self.on_move().await;
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
                self.on_move().await;
                // TODO: make sure the movement is allowed
            },
            DigBlock { status, location, face: _, sequence: _ } => {
                use minecraft_protocol::components::blocks::DiggingState;
                // TODO: Check legitimacy 
                if self.game_mode == Gamemode::Creative || status == DiggingState::Finished {
                    self.world.set_block(location.into(), BlockWithState::Air).await;
                }
            }
            packet => warn!("Unsupported packet received: {packet:?}"),
        }
    }
}

pub async fn handle_player(stream: TcpStream, player_info: PlayerInfo, mut server_msg_rcvr: BroadcastReceiver<ServerMessage>, world: Arc<World>, mut change_receiver: MpscReceiver<WorldChange>) -> Result<(), ()> {
    let (packet_sender, mut packet_receiver) = mpsc_channel(100);
    
    let mut handler = PlayerHandler {
        world,
        game_mode: Gamemode::Creative,
        position: Position { x: 0.0, y: 60.0, z: 0.0 },
        yaw: 0.0,
        pitch: 0.0,
        on_ground: false,
        packet_sender,

        center_chunk: ChunkPosition { cx: 0, cy: 11, cz: 0 },
        render_distance: player_info.render_distance.clamp(4, 15) as i32,
        loaded_chunks: HashSet::new(),

        info: player_info,
    };

    for cx in -3..=3 {
        for cz in -3..=3 {
            handler.loaded_chunks.insert(ChunkColumnPosition { cx, cz });
        }
    }

    let (mut reader_stream, mut writer_stream) = stream.into_split();
    
    let mut receive_packet_fut = Box::pin(receive_packet_split(&mut reader_stream).fuse());
    let mut receive_clientbound_fut = Box::pin(packet_receiver.recv().fuse());
    let mut receive_server_message_fut = Box::pin(server_msg_rcvr.recv().fuse());
    let mut receive_change_fut = Box::pin(change_receiver.recv().fuse());
    loop {
        // Select the first event that happens
        enum Event {
            PacketServerbound(Result<Vec<u8>, ()>),
            PacketClientbound(Option<Vec<u8>>),
            Message(Result<ServerMessage, BroadcastRecvError>),
            WorldChange(Option<WorldChange>),
        }
        let event = futures::select! {
            packet_serverbound = receive_packet_fut => Event::PacketServerbound(packet_serverbound),
            packet_clientbound = receive_clientbound_fut => Event::PacketClientbound(packet_clientbound),
            message = receive_server_message_fut => Event::Message(message),
            change = receive_change_fut => Event::WorldChange(change),
        };
        match event {
            Event::PacketServerbound(Ok(packet)) => {
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
            Event::WorldChange(Some(change)) => {
                drop(receive_change_fut);
                receive_change_fut = Box::pin(change_receiver.recv().fuse());

                match change {
                    WorldChange::BlockChange(position, block) => handler.on_block_change(position, block).await,
                }
            },
            Event::Message(Err(recv_error)) => {
                error!("Failed to receive message: {recv_error:?}");
                return Err(());
            }
            Event::PacketClientbound(None) => {
                error!("Failed to receive clientbound packet");
                return Err(());
            }
            Event::PacketServerbound(Err(e)) => {
                error!("Failed to receive serverbound packet: {e:?}");
                return Err(());
            }
            Event::WorldChange(None) => {
                error!("Failed to receive world change");
                return Err(());
            }
        }
    }
}
