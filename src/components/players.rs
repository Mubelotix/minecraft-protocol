use crate::*;

#[derive(Debug)]
pub struct PlayersInfos<'a> {
    pub players_infos: Vec<PlayerInfos<'a>>
}

#[derive(Debug)]
pub struct PlayerInfos<'a> {
    pub player_uuid: UUID,
    pub actions: Vec<PlayerActions<'a>>,
}

#[derive(Debug)]
#[repr(u8)]
pub enum PlayerActions<'a> {
    AddPlayer(AddPlayersAction<'a>) = 0x01,
    InitializeChat(InitializeChatAction<'a>) = 0x02,
    UpdateGamemodes(UpdateGamemodesAction) = 0x04,
    UpdateListed(UpdateListedAction) = 0x08,
    Ping(PingAction) = 0x10,
    UpdateDisplayName(UpdateDisplayNameAction<'a>) = 0x20,
}

impl<'a> PlayerActions<'a> {
    pub fn get_descriminent(&self) -> u8 {
        match self {
            PlayerActions::AddPlayer(_) => 0x01,
            PlayerActions::InitializeChat(_) => 0x02,
            PlayerActions::UpdateGamemodes(_) => 0x04,
            PlayerActions::UpdateListed(_) => 0x08,
            PlayerActions::Ping(_) => 0x10,
            PlayerActions::UpdateDisplayName(_) => 0x20,
        }
    }
}

#[derive(Debug, MinecraftPacketPart)]
pub struct AddPlayersAction<'a> {
    name: &'a str,
    properties: Array<'a, Property<'a>, VarInt>,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct InitializeChatAction<'a> {
    initialize_chat: Option<InitializeChat<'a>>,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct UpdateGamemodesAction {
    gamemode: VarInt,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct UpdateListedAction {
    /// Whether the player should be listed on the player list.
    listed: bool,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct PingAction {
    /// Measured in milliseconds
    ping: VarInt,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct UpdateDisplayNameAction<'a> {
    display_name: Option<Chat<'a>>,
}

/// The Property field looks as in the response of M[ojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape), except of course using the protocol format instead of JSON.
/// That is, each player will usually have one property with Name “textures” and Value being a base64-encoded JSON string as documented at [Mojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape).
/// An empty properties array is also acceptable, and will cause clients to display the player with one of the two default skins depending on UUID.
#[derive(Debug, MinecraftPacketPart)]
pub struct Property<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub signature: Option<&'a str>,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct InitializeChat<'a> {
    pub session_id: UUID,
    /// Key expiry time, as a UNIX timestamp in milliseconds. 
    pub pub_key_expire_time: i64,
    /// The player's public key, in bytes. 
    pub encoded_pub_key: Array<'a, u8, VarInt>,
    /// The public key's digital signature.
    pub pub_key_signature: Array<'a, u8, VarInt>,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct PlayerAdditionInfo<'a> {
    pub uuid: UUID,
    pub name: &'a str,
    /// The Property field looks as in the response of M[ojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape), except of course using the protocol format instead of JSON.
    /// That is, each player will usually have one property with Name “textures” and Value being a base64-encoded JSON string as documented at [Mojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape).
    /// An empty properties array is also acceptable, and will cause clients to display the player with one of the two default skins depending on UUID.
    pub properties: Array<'a, Property<'a>, VarInt>,
    pub gamemode: super::gamemode::Gamemode,
    /// Measured in milliseconds
    pub ping: VarInt,
    pub display_name: Option<Chat<'a>>,
}


#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum FaceAim {
    Feet,
    Eyes,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct FaceTarget {
    pub target_entity_id: VarInt,
    pub target_aim: FaceAim,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct DeathLocation<'a> {
    pub dimension: Identifier<'a>,
    pub position: Position,
}

impl<'a> MinecraftPacketPart<'a> for PlayersInfos<'a> {
    /// Look at this [wiki page](https://wiki.vg/Protocol#Player_Info_Update) for more information about this packet.
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let mask =  {
            // If no player action is sent, the mask is 0.
            if self.players_infos.is_empty() {
                0u8
            } else {
                // The mask is the sum of all the player actions. The variants of the player actions are the same for all players.
                // So we can just take the first player and sum all the variants.
                let first_player = &self.players_infos[0];
                first_player.actions.iter().fold(0, |acc, action| acc + action.get_descriminent())
            }
        };
        let n_players = self.players_infos.len();

        // Write the mask
        mask.serialize_minecraft_packet_part(output)?;
        // Write the number of players (array size)
        VarInt::from(n_players).serialize_minecraft_packet_part(output)?;
        // Write the length of the following array
        for player_info in self.players_infos {
            // Write the uuid of the concerned player
            player_info.player_uuid.serialize_minecraft_packet_part(output)?;
            
            // Write the actions here and not in the PlayerInfos struct because the mask defines the action type
            // As the mask is not a prefix, we can't use the trait to deserialize the actions so we can't implement the trait for PlayerInfos
            for action in player_info.actions {
                match action {
                    PlayerActions::AddPlayer(action) => action.serialize_minecraft_packet_part(output)?,
                    PlayerActions::InitializeChat(action) => action.serialize_minecraft_packet_part(output)?,
                    PlayerActions::UpdateGamemodes(action) => action.serialize_minecraft_packet_part(output)?,
                    PlayerActions::UpdateListed(action) => action.serialize_minecraft_packet_part(output)?,
                    PlayerActions::Ping(action) => action.serialize_minecraft_packet_part(output)?,
                    PlayerActions::UpdateDisplayName(action) => action.serialize_minecraft_packet_part(output)?,
                }
            }
        }

        Ok(())
    }

    /// Look at this [wiki page](https://wiki.vg/Protocol#Player_Info_Update) for more information about this packet.
    fn deserialize_minecraft_packet_part(input: &'a [u8])
        -> Result<(Self, &'a [u8]), &'static str> {
        // The first byte is the mask of actions
        let (mut mask, input) = u8::deserialize_minecraft_packet_part(input)?;
        // The second byte is the number of players
        let (n_players, mut input) = VarInt::deserialize_minecraft_packet_part(input)?;
        // We will deserialize n_players times the player uuid and actions 
        let mut players_infos = Vec::with_capacity(n_players.0 as usize);
        for _ in 0..n_players.0 {
            // The first part of the player infos is the uuid
            let (player_uuid, new_input) = UUID::deserialize_minecraft_packet_part(input)?;
            input = new_input;
            // The second part is the actions
            let mut actions = Vec::new();
            
            // the least significant bit of the mask corresponds to the first action
            // the most significant bit of the mask corresponds to the last action
            // So we need to iterate over the bits of the mask to know which actions are present
            let mut current_bit = 0;
            while mask > 0 {
                // If the bit is 1, the action is present
                if (mask & 1) > 0 {
                    // We need to deserialize the action
                    let (action, new_input) = match current_bit {
                        0 => {
                            let (action, input) = AddPlayersAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::AddPlayer(action), input)
                        },
                        1 => {
                            let (action, input) = InitializeChatAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::InitializeChat(action), input)
                        },
                        2 => {
                            let (action, input) = UpdateGamemodesAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::UpdateGamemodes(action), input)
                        },
                        3 => {
                            let (action, input) = UpdateListedAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::UpdateListed(action), input)
                        },
                        4 => {
                            let (action, input) = PingAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::Ping(action), input)
                        },
                        5 => {
                            let (action, input) = UpdateDisplayNameAction::deserialize_minecraft_packet_part(input)?;
                            (PlayerActions::UpdateDisplayName(action), input)
                        },
                        _ => return Err("Invalid mask"),
                    };
                    input = new_input;
                    actions.push(action);
                }
                mask >>= 1;
                current_bit += 1;
            }
            players_infos.push(PlayerInfos { player_uuid, actions });
        }
            
        Ok((Self { players_infos }, input))
    }
}
