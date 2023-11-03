use super::*;

pub struct PlayerInfo {
    pub(super) addr: SocketAddr,
    pub(super) username: String,
    pub(super) uuid: u128,
    pub(super) locale: String,
    pub(super) render_distance: usize,
    pub(super) chat_mode: ChatMode,
    pub(super) chat_colors: bool,
    pub(super) displayed_skin_parts: u8,
    pub(super) main_hand: MainHand,
    pub(super) enable_text_filtering: bool,
    pub(super) allow_server_listing: bool,
}

pub async fn handshake(stream: &mut TcpStream, logged_in_player_info: LoggedInPlayerInfo) -> Result<PlayerInfo, ()> {
    // Receive client informations
    let packet = receive_packet(stream).await;
    debug!("Packet received");
    let packet = ConfigServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    let ConfigServerbound::ClientInformations { locale, render_distance, chat_mode, chat_colors, displayed_skin_parts, main_hand, enable_text_filtering, allow_server_listing } = packet else {
        error!("Expected ClientInformation packet, got: {packet:?}");
        return Err(());
    };
    debug!("ClientInformation received");

    // Send server agent
    let server_agent = ConfigClientbound::PluginMessage {
        channel: "minecraft:brand",
        data: RawBytes {
            data: &[6, 83, 112, 105, 103, 111, 116]
        },
    };
    send_packet(stream, server_agent).await;
    debug!("PluginMessage sent");

    // Send feature flags
    let feature_flags = ConfigClientbound::FeatureFlags {
        features: Array::from(vec!["minecraft:vanilla"]),
    };
    send_packet(stream, feature_flags).await;
    debug!("FeatureFlags sent");

    // Send registry data
    send_packet_raw(stream, include_bytes!("../raw/registry_codec.mc_packet")).await;
    debug!("RegistryData sent");

    // Update tags
    let update_tags = ConfigClientbound::UpdateTags {
        tags: Map::default(),
    };
    send_packet(stream, update_tags).await;
    debug!("UpdateTags sent");

    // Send finish configuration
    let finish_configuration = ConfigClientbound::FinishConfiguration;
    send_packet(stream, finish_configuration).await;
    debug!("FinishConfiguration sent");

    // Receive finish configuration
    let packet = receive_packet(stream).await;
    let packet = ConfigServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    let ConfigServerbound::FinishConfiguration = packet else {
        error!("Expected FinishConfiguration packet, got: {packet:?}");
        return Err(());
    };
    debug!("FinishConfiguration received");

    // Send join game
    let player_id: usize = 3429; // TODO dynamic attribution
    let join_game = PlayClientbound::JoinGame {
        player_id: player_id as i32,
        is_hardcore: false,
        dimensions_names: Array::from(vec!["minecraft:overworld"]),
        max_players: VarInt::from(1000),
        render_distance: VarInt::from(12),
        simulation_distance: VarInt::from(8),
        reduced_debug_info: false,
        enable_respawn_screen: true,
        do_limited_crafting: false,
        dimension_type: "minecraft:overworld",
        dimension_name: "minecraft:overworld",
        hashed_seed: 42,
        gamemode: Gamemode::Creative,
        previous_gamemode: PreviousGamemode::Creative,
        is_debug: false,
        is_flat: true,
        death_location: None,
        portal_cooldown: VarInt::from(0),
    };
    send_packet(stream, join_game).await;
    debug!("JoinGame sent");

    // Set difficulty
    let change_difficulty = PlayClientbound::ChangeDifficulty {
        difficulty: Difficulty::Normal,
        difficulty_locked: false
    };
    send_packet(stream, change_difficulty).await;
    debug!("ChangeDifficulty sent");

    // Set player abilities
    let change_player_abilities = PlayClientbound::PlayerAbilities {
        flags: 0,
        flying_speed: 0.05,
        field_of_view_modifier: 0.1
    };
    send_packet(stream, change_player_abilities).await;
    debug!("PlayerAbilities sent");

    // Set held item
    let held_item_change = PlayClientbound::SetHeldItem {
        slot: 0 // TODO should be the same as when disconnected
    };
    send_packet(stream, held_item_change).await;
    debug!("SetHeldItem sent");

    // Update recipes
    let update_recipes = PlayClientbound::UpdateRecipes {
        data: RawBytes {
            data: &[0]
        }
    };
    send_packet(stream, update_recipes).await;
    debug!("UpdateRecipes sent");

    // Entity event
    let entity_event = PlayClientbound::EntityEvent {
        entity_id: player_id as i32,
        entity_status: 28
    };
    send_packet(stream, entity_event).await;
    debug!("EntityEvent sent");

    // Declare commands
    let declare_commands = PlayClientbound::DeclareCommands {
        count: VarInt(0),
        data: RawBytes {
            data: &[0]
        }
    };
    send_packet(stream, declare_commands).await;
    debug!("DeclareCommands sent");

    // Unlock recipes
    let unlock_recipes = PlayClientbound::UnlockRecipes {
        action: minecraft_protocol::components::recipes::UnlockRecipesAction::Init {
            crafting_recipe_book_open: false,
            crafting_recipe_book_filter_active: false,
            smelting_recipe_book_open: false,
            smelting_recipe_book_filter_active: false,
            blast_furnace_recipe_book_open: false,
            blast_furnace_recipe_book_filter_active: false,
            smoker_recipe_book_open: false,
            smoker_recipe_book_filter_active: false,
            displayed_recipes: Array::default(),
            added_recipes: Array::default()
        }
    };
    send_packet(stream, unlock_recipes).await;
    debug!("UnlockRecipes sent");

    // Spawn player
    let player_position = PlayClientbound::PlayerPositionAndLook {
        x: 0.0,
        y: 60.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
        teleport_id: VarInt(1),
    };
    send_packet(stream, player_position).await;
    debug!("PlayerPositionAndLook sent");

    // Send server metadata
    let server_data = PlayClientbound::ServerData {
        motd: "{\"text\":\"A Minecraft Server\"}",
        icon: None,
        enforces_secure_chat: false,
    };
    send_packet(stream, server_data).await;
    debug!("ServerData sent");

    // Spawn message
    let spawn_message = PlayClientbound::SystemChatMessage {
        content: "{\"text\":\"Hello world\"}",
        overlay: false,
    };
    send_packet(stream, spawn_message).await;
    debug!("SystemChatMessage sent");

    // TODO: update players info (x2)

    // Set entity metadata
    let mut entity_metadata = BTreeMap::new();
    entity_metadata.insert(9, EntityMetadataValue::Float { value: 20.0 });
    entity_metadata.insert(16, EntityMetadataValue::VarInt { value: VarInt(18) });
    entity_metadata.insert(17, EntityMetadataValue::Byte { value: 127 });
    let set_entity_metadata = PlayClientbound::SetEntityMetadata {
        entity_id: VarInt::from(player_id),
        metadata: EntityMetadata { items: entity_metadata.clone() }
    };
    send_packet(stream, set_entity_metadata).await;
    debug!("SetEntityMetadata sent");

    // Initialize world border
    let world_border_init = PlayClientbound::InitializeWorldBorder {
        x: 0.0,
        y: 0.0,
        old_diameter: 60000000.0,
        new_diameter: 60000000.0,
        speed: VarLong(0),
        portal_teleport_boundary: VarInt(29999984),
        warning_blocks: VarInt(5),
        warning_time: VarInt(15),
    };
    send_packet(stream, world_border_init).await;
    debug!("InitializeWorldBorder sent");

    // Update time
    let time_update = PlayClientbound::UpdateTime {
        world_age: 0,
        time_of_day: 0,
    };
    send_packet(stream, time_update).await;
    debug!("UpdateTime sent");

    // Set spawn position
    let set_spawn_position = PlayClientbound::SetSpawnPosition {
        location: minecraft_protocol::packets::Position { x: 0, y: 70, z: 0 },
        angle: 0.0,
    };
    send_packet(stream, set_spawn_position).await;
    debug!("SetSpawnPosition sent");

    // Set center chunk
    let set_center_chunk = PlayClientbound::SetCenterChunk {
        chunk_x: VarInt(0), // TODO: should be the same as when disconnected
        chunk_z: VarInt(0), // TODO: should be the same as when disconnected
    };
    send_packet(stream, set_center_chunk).await;
    debug!("SetCenterChunk sent");

    // Set inventory
    let set_container_content = PlayClientbound::SetContainerContent {
        window_id: 0,
        state_id: VarInt(1),
        slots: Array::default(),
        carried_item: Slot { item: None }
    };
    send_packet(stream, set_container_content).await;
    debug!("SetContainerContent sent");

    // Set entity metadata (again)
    let set_entity_metadata = PlayClientbound::SetEntityMetadata {
        entity_id: VarInt::from(player_id),
        metadata: EntityMetadata { items: entity_metadata }
    };
    send_packet(stream, set_entity_metadata).await;
    debug!("SetEntityMetadata sent");

    // Update entity attributes
    let mut entity_attributes = BTreeMap::new();
    entity_attributes.insert("minecraft:generic.attack_speed", EntityAttribute { value: 4.0, modifiers: Array::default() });
    entity_attributes.insert("minecraft:generic.max_health", EntityAttribute { value: 20.0, modifiers: Array::default() });
    entity_attributes.insert("minecraft:generic.movement_speed", EntityAttribute { value: 0.10000000149011612, modifiers: Array::default() });
    let update_entity_attributes = PlayClientbound::UpdateEntityAttributes {
        entity_id: VarInt::from(player_id),
        attributes: Map::from(entity_attributes)
    };
    send_packet(stream, update_entity_attributes).await;
    debug!("UpdateEntityAttributes sent");

    // Update advancements
    let update_advancements = PlayClientbound::UpdateAdvancements {
        reset: true,
        advancement_mapping: Map::default(),
        advancements_to_remove: Array::default(),
        progress_mapping: Map::default(),
    };
    send_packet(stream, update_advancements).await;
    debug!("UpdateAdvancements sent");

    // Set health
    let set_health = PlayClientbound::SetHealth {
        health: 20.0,
        food: VarInt(20),
        food_saturation: 5.0,
    };
    send_packet(stream, set_health).await;
    debug!("UpdateHealth sent");

    // Set experience
    let set_experience = PlayClientbound::SetExperience {
        experience_level: VarInt(0),
        experience_bar: 0.0,
        total_experience: VarInt(0),
    };
    send_packet(stream, set_experience).await;
    debug!("SetExperience sent");

    // Chunk batch start
    let chunk_data = PlayClientbound::ChunkBatchStart;
    send_packet(stream, chunk_data).await;
    debug!("ChunkBatchStart sent");

    let empty_chunk = NetworkChunk {
        block_count: 0,
        blocks: PalettedData::Single { value: 0 },
        biomes: PalettedData::Single { value: 4 },
    };
    let dirt_chunk = NetworkChunk {
        block_count: 4096,
        blocks: PalettedData::Single { value: minecraft_protocol::ids::blocks::Block::GrassBlock.default_state_id() },
        biomes: PalettedData::Single { value: 4 },
    };
    let mut flat_column = Vec::new();
    flat_column.push(dirt_chunk);
    for _ in 0..23 {
        flat_column.push(empty_chunk.clone());
    }
    let serialized: Vec<u8> = NetworkChunk::into_data(flat_column).unwrap();
    let mut heightmaps = HashMap::new();
    heightmaps.insert(String::from("MOTION_BLOCKING"), NbtTag::LongArray(vec![0; 37]));
    let heightmaps = NbtTag::Compound(heightmaps);
    
    for cx in -3..=3 {
        for cz in -3..=3 {
            let chunk_data = PlayClientbound::ChunkData {
                value: ChunkData {
                    chunk_x: cx,
                    chunk_z: cz,
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
            send_packet(stream, chunk_data).await;
        }
    }
    debug!("ChunkData sent");

    // Chunk batch end
    let chunk_data = PlayClientbound::ChunkBatchFinished { batch_size: VarInt(49) };
    send_packet(stream, chunk_data).await;
    debug!("ChunkBatchFinished sent");

    // Get chunk batch acknoledgement
    let packet = receive_packet(stream).await;
    let packet = PlayServerbound::deserialize_uncompressed_minecraft_packet(packet.as_slice()).unwrap();
    let PlayServerbound::ChunkBatchReceived { chunks_per_tick } = packet else {
        error!("Expected ChunkBatchAcknoledgement packet, got: {packet:?}");
        return Err(());
    };
    debug!("ChunkBatchAcknoledgement received");

    Ok(PlayerInfo {
        addr: logged_in_player_info.addr,
        username: logged_in_player_info.username,
        uuid: logged_in_player_info.uuid,
        locale: locale.to_owned(),
        render_distance: render_distance.try_into().unwrap_or(5),
        chat_mode,
        chat_colors,
        displayed_skin_parts,
        main_hand,
        enable_text_filtering,
        allow_server_listing,
    })
}
