use super::*;

#[MinecraftEntity(
    ancestors { LivingEntity, Entity },
    defines {
        Entity.init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>);
    }
)]
pub struct Player {
    pub living_entity: LivingEntity,
    pub additional_hearts: usize,
    pub score: usize,
    pub is_cape_enabled: bool,
    pub is_jacket_enabled: bool,
    pub is_left_sleeve_enabled: bool,
    pub is_right_sleeve_enabled: bool,
    pub is_left_pants_leg_enabled: bool,
    pub is_right_pants_leg_enabled: bool,
    pub is_hat_enabled: bool,
    pub left_shoulder_entity: NbtTag,
    pub right_shoulder_entity: NbtTag,

    game_mode: Gamemode,
    info: PlayerInfo,
    on_ground: bool,
    packet_sender: MpscSender<Vec<u8>>,
    entity_prev_positions: HashMap<Eid, Position>,
    render_distance: i32,
    loaded_chunks: HashSet<ChunkColumnPosition>,
    center_chunk: ChunkPosition,
    packets_sent: usize,
}

impl Player {
    pub async fn spawn_player(
        world: &'static World,
        stream: TcpStream,
        player_info: PlayerInfo,
        server_msg_rcvr: BroadcastReceiver<ServerMessage>,
        change_receiver: MpscReceiver<WorldChange>
    ) -> Eid {
        let (packet_sender, packet_receiver) = mpsc_channel(1000);
        let uuid = player_info.uuid;

        let mut player = Player {
            living_entity: LivingEntity::default(),
            additional_hearts: 0,
            score: 0,
            is_cape_enabled: false,
            is_jacket_enabled: false,
            is_left_sleeve_enabled: false,
            is_right_sleeve_enabled: false,
            is_left_pants_leg_enabled: false,
            is_right_pants_leg_enabled: false,
            is_hat_enabled: false,
            left_shoulder_entity: NbtTag::Null,
            right_shoulder_entity: NbtTag::Null,

            game_mode: Gamemode::Creative,
            on_ground: false,
            packet_sender,
    
            entity_prev_positions: HashMap::new(),
    
            center_chunk: ChunkPosition { cx: 0, cy: 11, cz: 0 },
            render_distance: player_info.render_distance.clamp(4, 15) as i32,
            loaded_chunks: HashSet::new(),
    
            info: player_info,
            packets_sent: 0,
        };
        
        // TODO: player should load existing entities
        
        for cx in -3..=3 {
            for cz in -3..=3 {
                player.loaded_chunks.insert(ChunkColumnPosition { cx, cz });
            }
        }
        
        let eid = world.spawn_entity::<Player>(AnyEntity::Player(player)).await;
        let handler = Handler::assume(eid, world);
        handler.clone().insert_task("player", tokio::spawn(handle_player(handler, uuid, stream, packet_receiver, server_msg_rcvr, change_receiver))).await;

        eid
    }
}

impl Handler<Player> {
    #[instrument(skip_all)]
    async fn update_center_chunk(self) {
        let Some((old_center_chunk, new_center_chunk, render_distance)) = self.mutate(|player| {
            let old_center_chunk = player.center_chunk.clone();
            let new_center_chunk = player.get_entity().position.chunk();
            player.center_chunk = new_center_chunk.clone();
            ((old_center_chunk, new_center_chunk, player.render_distance), EntityChanges::other())
        }).await else {return};

        // Tell the client which chunk he is in
        if new_center_chunk == old_center_chunk { return };
        self.send_packet(PlayClientbound::SetCenterChunk { chunk_x: VarInt(new_center_chunk.cx), chunk_z: VarInt(new_center_chunk.cz) }).await;

        // Find out which chunks should be loaded
        if new_center_chunk.chunk_column() == old_center_chunk.chunk_column() { return };
        let mut loaded_chunks_after = HashSet::new();
        for cx in (new_center_chunk.cx - render_distance)..=(new_center_chunk.cx + render_distance) {
            for cz in (new_center_chunk.cz - render_distance)..=(new_center_chunk.cz + render_distance) {
                let dist = (((cx - new_center_chunk.cx).pow(2) + (cz - new_center_chunk.cz).pow(2)) as f32).sqrt();
                if dist > render_distance as f32 { continue };
                loaded_chunks_after.insert(ChunkColumnPosition { cx, cz });
            }
        }

        // Select chunks to load (max 50) and unload
        let Some((loaded_chunks_after, newly_loaded_chunks, unloaded_chunks, uuid)) = self.mutate(|player| {
            if loaded_chunks_after == player.loaded_chunks { return (None, EntityChanges::nothing()) };
            let mut newly_loaded_chunks: Vec<_> = loaded_chunks_after.difference(&player.loaded_chunks).cloned().collect();
            let unloaded_chunks: Vec<_> = player.loaded_chunks.difference(&loaded_chunks_after).cloned().collect();
            for skipped in newly_loaded_chunks.iter().skip(50) {
                loaded_chunks_after.remove(skipped);
            }
            newly_loaded_chunks.truncate(50);
            let uuid = player.info.uuid;
            player.loaded_chunks = loaded_chunks_after.clone();
            (Some((loaded_chunks_after, newly_loaded_chunks, unloaded_chunks, uuid)), EntityChanges::other())
        }).await.flatten() else { return };

        // Tell the world about the changes
        self.world.update_loaded_chunks(uuid, loaded_chunks_after).await;

        // Send the chunks to the client
        let mut heightmaps = HashMap::new();
        heightmaps.insert(String::from("MOTION_BLOCKING"), NbtTag::LongArray(vec![0; 37]));
        let heightmaps = NbtTag::Compound(heightmaps);
        for newly_loaded_chunk in newly_loaded_chunks {
            let mut column = Vec::new();
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
                    heightmaps: heightmaps.clone(),
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
    }

    #[instrument(skip_all)]
    async fn send_packet<'a>(&self, packet: PlayClientbound<'a>) {
        let packet = packet.serialize_minecraft_packet().unwrap();
        let packets_sent = self.mutate(|player| {
            player.packets_sent += 1;
            (player.packets_sent, EntityChanges::other())
        }).await.unwrap_or(0);
        if packets_sent > 500 {
            warn!("Many packets sent ({packets_sent})");
        }
        let Some(packet_sender) = self.observe(|player| player.packet_sender.clone()).await else {return};
        packet_sender.send(packet).await.unwrap();
    }

    #[instrument(skip_all)]
    async fn on_server_message(self, message: ServerMessage) {
        use ServerMessage::*;
        match message {
            Tick(tick_id) => {
                let span = info_span!("player tick");
                let _enter = span.enter();

                if tick_id % (20*10) == 0 {
                    self.send_packet(PlayClientbound::KeepAlive { keep_alive_id: tick_id as u64 }).await;
                }
                self.mutate(|player| {
                    player.packets_sent = 0;
                    (player.packets_sent, EntityChanges::other())
                }).await;
                self.send_packet(PlayClientbound::BundleDelimiter).await;
            }
        }
    }

    #[instrument(skip_all)]
    async fn on_world_change(self, change: WorldChange) {
        match change {
            WorldChange::Block(position, block) => {
                self.send_packet(PlayClientbound::BlockUpdate {
                    location: position.into(),
                    block_state: block,
                }).await;
            },
            WorldChange::EntitySpawned { eid, uuid, ty, position, pitch, yaw, head_yaw, data, velocity, metadata } => {
                self.mutate(|player| {player.entity_prev_positions.insert(eid, position.clone()); ((), EntityChanges::other())}).await;
                self.send_packet(PlayClientbound::SpawnEntity {
                    id: VarInt(eid as i32),
                    uuid,
                    entity_type: ty,
                    x: position.x,
                    y: position.y,
                    z: position.z,
                    pitch: (pitch * (256.0 / 360.0)) as u8,
                    yaw: (yaw * (256.0 / 360.0)) as u8,
                    head_yaw: (head_yaw * (256.0 / 360.0)) as u8,
                    data: VarInt(data as i32),
                    velocity_x: (velocity.x * 8000.0) as i16,
                    velocity_y: (velocity.y * 8000.0) as i16,
                    velocity_z: (velocity.z * 8000.0) as i16,
                }).await;
            },
            WorldChange::EntityDispawned { eid } => {
                self.mutate(|player| {player.entity_prev_positions.remove(&eid); ((), EntityChanges::other())}).await;
                self.send_packet(PlayClientbound::RemoveEntities { entity_ids: Array::from(vec![VarInt(eid as i32)]) }).await;
            },
            WorldChange::EntityMetadata { eid, metadata } => todo!(),
            WorldChange::EntityPosition { eid, position } => {
                let Some(prev_position) = self.mutate(|player| ((player.entity_prev_positions.insert(eid, position.clone())), EntityChanges::other())).await else {return};
                match prev_position {
                    Some(prev_position) => {
                        self.send_packet(PlayClientbound::UpdateEntityPosition {
                            entity_id: VarInt(eid as i32),
                            delta_x: (position.x * 4096.0 - prev_position.x * 4096.0) as i16,
                            delta_y: (position.y * 4096.0 - prev_position.y * 4096.0) as i16,
                            delta_z: (position.z * 4096.0 - prev_position.z * 4096.0) as i16,
                            on_ground: true, // TODO add on_ground in entity position
                        }).await;
                    }
                    None => {
                        let Some((ty, velocity, pitch, yaw, head_yaw)) = self.world.observe_entity(eid, |any_entity| {
                            let ty = any_entity.to_network().unwrap(); // TODO: handle non-networkable entities
                            let entity = any_entity.as_entity();
                            let velocity = entity.velocity.clone();
                            let pitch = entity.pitch;
                            let yaw = entity.yaw;
                            let living_entity = TryAsEntityRef::<LivingEntity>::try_as_entity_ref(any_entity);
                            let head_pitch = living_entity.map(|living| living.head_yaw).unwrap_or(0.0);
                            (ty, velocity, pitch, yaw, head_pitch)
                        }).await else {return};
                        let uuid = 0; // TODO set uuid
                        self.send_packet(PlayClientbound::SpawnEntity {
                            id: VarInt(eid as i32),
                            uuid,
                            entity_type: ty,
                            x: position.x,
                            y: position.y,
                            z: position.z,
                            pitch: (pitch * (256.0 / 360.0)) as u8,
                            yaw: (yaw * (256.0 / 360.0)) as u8,
                            head_yaw: (head_yaw * (256.0 / 360.0)) as u8,
                            data: VarInt(0 as i32), // TODO set data on entities
                            velocity_x: (velocity.x * 8000.0) as i16,
                            velocity_y: (velocity.y * 8000.0) as i16,
                            velocity_z: (velocity.z * 8000.0) as i16,
                        }).await;
                    }
                }
            },
            WorldChange::EntityVelocity { eid, velocity } => {
                self.send_packet(PlayClientbound::SetEntityVelocity {
                    entity_id: VarInt(eid as i32),
                    velocity_x: (velocity.x * 8000.0) as i16,
                    velocity_y: (velocity.y * 8000.0) as i16,
                    velocity_z: (velocity.z * 8000.0) as i16,
                }).await;
            },
            WorldChange::EntityPitch { eid, pitch, yaw, head_yaw } => {
                self.send_packet(PlayClientbound::UpdateEntityRotation {
                    entity_id: VarInt(eid as i32),
                    yaw: (yaw * (256.0 / 360.0)) as u8,
                    pitch: (pitch * (256.0 / 360.0)) as u8,
                    on_ground: true, // TODO add on_ground in entity position
                }).await;
            },
        }
    }

    #[instrument(skip_all)]
    async fn on_packet<'a>(mut self, packet: PlayServerbound<'a>) {
        use PlayServerbound::*;
        match packet {
            SetPlayerPosition { x, y, z, on_ground } => {
                self.mutate(|player| {
                    let entity = player.get_entity_mut();
                    entity.position.x = x;
                    entity.position.y = y;
                    entity.position.z = z;
                    player.on_ground = on_ground;
                    ((), EntityChanges::position())
                }).await;
                self.update_center_chunk().await;
                // TODO: make sure the movement is allowed
            },
            SetPlayerRotation { yaw, pitch, on_ground } => {
                self.mutate(|player| {
                    let entity = player.get_entity_mut();
                    entity.yaw = yaw;
                    entity.pitch = pitch;
                    player.on_ground = on_ground;
                    ((), EntityChanges::pitch())
                }).await;
            }
            SetPlayerPositionAndRotation { x, y, z, yaw, pitch, on_ground } => {
                self.mutate(|player| {
                    let entity = player.get_entity_mut();
                    entity.position.x = x;
                    entity.position.y = y;
                    entity.position.z = z;
                    entity.yaw = yaw;
                    entity.pitch = pitch;
                    player.on_ground = on_ground;
                    ((), EntityChanges::position()+EntityChanges::pitch())
                }).await;
                self.update_center_chunk().await;
                // TODO: make sure the movement is allowed
            },
            DigBlock { status, location, face: _, sequence: _ } => {
                use minecraft_protocol::components::blocks::DiggingState;
                // TODO: Check legitimacy
                let Some(game_mode) = self.observe(|player| player.game_mode.clone()).await else {return};
                if game_mode == Gamemode::Creative || status == DiggingState::Finished {
                    self.world.set_block(location.into(), BlockWithState::Air).await;
                }
            }
            ChatMessage { message, .. } => {
                if message == "summon" {
                    let mut zombie = Zombie::default();
                    let Some(mut position) = self.observe(|player| player.get_entity().position.clone()).await else {return};
                    position.y += 20.0;
                    zombie.get_entity_mut().position = position;
                    self.world.spawn_entity::<Zombie>(AnyEntity::Zombie(zombie)).await;
                } else if message == "stress" {
                    tokio::spawn(async move {
                        for i in 0..1000 {
                            let mut zombie = Zombie::default();
                            let Some(mut position) = self.observe(|player| player.get_entity().position.clone()).await else {return};
                            position.y += 20.0;
                            zombie.get_entity_mut().position = position;
                            self.world.spawn_entity::<Zombie>(AnyEntity::Zombie(zombie)).await;
                            tokio::time::sleep(Duration::from_millis(50)).await;
                        }
                    });
                }
            }
            RequestPing { payload } => {
                self.send_packet(PlayClientbound::Ping { id: payload as i32 }).await;
            }
            packet => warn!("Unsupported packet received: {packet:?}"),
        }
    }
}

#[instrument(skip_all)]
async fn handle_player(h: Handler<Player>, uuid: UUID, stream: TcpStream, packet_receiver: MpscReceiver<Vec<u8>>, server_msg_rcvr: BroadcastReceiver<ServerMessage>, change_receiver: MpscReceiver<WorldChange>) {
    let r = handle_player_inner(h.clone(), stream, packet_receiver, server_msg_rcvr, change_receiver).await;
    match r {
        Ok(()) => info!("Player handler shut down gracefully"),
        Err(()) => error!("Player handler crashed")
    }
    h.world.remove_loader(uuid).await;
}

#[instrument(skip_all)]
async fn handle_player_inner(h: Handler<Player>, stream: TcpStream, mut packet_receiver: MpscReceiver<Vec<u8>>, mut server_msg_rcvr: BroadcastReceiver<ServerMessage>, mut change_receiver: MpscReceiver<WorldChange>) -> Result<(), ()> {
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
                h.clone().on_packet(packet).await;
            },
            Event::PacketClientbound(Some(packet)) => {
                drop(receive_clientbound_fut);
                receive_clientbound_fut = Box::pin(packet_receiver.recv().fuse());

                send_packet_raw_split(&mut writer_stream, packet.as_slice()).await;
            },
            Event::Message(Ok(message)) => {
                drop(receive_server_message_fut);
                receive_server_message_fut = Box::pin(server_msg_rcvr.recv().fuse());

                h.clone().on_server_message(message).await;
            },
            Event::WorldChange(Some(change)) => {
                drop(receive_change_fut);
                receive_change_fut = Box::pin(change_receiver.recv().fuse());

                h.clone().on_world_change(change).await;
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

impl Handler<Player> {
    pub async fn init(self, server_msg_rcvr: BroadcastReceiver<ServerMessage>) {
        //self.insert_task("newton", tokio::spawn(newton_task(self.clone(), server_msg_rcvr))).await;
    }
}
