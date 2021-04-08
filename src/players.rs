use crate::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum PlayerInfoAction<'a> {
    AddPlayers {
        modifications: Array<'a, PlayerAdditionInfo<'a>, VarInt>,
    },
    UpdateGamemodes {
        modifications: Array<'a, PlayerGamemodeChangeInfo<'a>, VarInt>,
    },
    UpdateLatencies {
        modifications: Array<'a, PlayerLatencyUpdateInfo<'a>, VarInt>,
    },
    UpdateDisplayNames {
        modifications: Array<'a, PlayerDisplayNameChangeInfo<'a>, VarInt>,
    },
    RemovePlayers {
        players_to_remove: Array<'a, &'a str, VarInt>,
    },
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
pub struct PlayerAdditionInfo<'a> {
    pub name: &'a str,
    /// The Property field looks as in the response of M[ojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape), except of course using the protocol format instead of JSON.
    /// That is, each player will usually have one property with Name “textures” and Value being a base64-encoded JSON string as documented at [Mojang API#UUID -> Profile + Skin/Cape](https://wiki.vg/Mojang_API#UUID_-.3E_Profile_.2B_Skin.2FCape).
    /// An empty properties array is also acceptable, and will cause clients to display the player with one of the two default skins depending on UUID.
    pub properties: Array<'a, Property<'a>, VarInt>,
    pub gamemode: crate::gamemode::Gamemode,
    /// Measured in milliseconds
    pub ping: VarInt,
    pub display_name: Option<Chat<'a>>,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct PlayerGamemodeChangeInfo<'a> {
    pub name: &'a str,
    pub gamemode: crate::gamemode::Gamemode,
}

#[derive(Debug, MinecraftPacketPart)]
pub struct PlayerLatencyUpdateInfo<'a> {
    pub name: &'a str,
    /// Measured in milliseconds
    pub ping: VarInt,
}
#[derive(Debug, MinecraftPacketPart)]
pub struct PlayerDisplayNameChangeInfo<'a> {
    pub name: &'a str,
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
