use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum ScoreboardAction<'a> {
    Create {
        text: Chat<'a>,
        scoreboard_type: ScoreboardType,
    },
    Remove,
    Update {
        text: Chat<'a>,
        scoreboard_type: ScoreboardType,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
#[discriminant(u8)]
pub enum ScoreboardScoreAction<'a> {
    /// Update or Create
    Update {
        /// The name of the objective the score belongs to
        objective_name: Chat<'a>,
        /// The score to be displayed next to the entry
        value: VarInt,
    },
    Remove {
        /// The name of the objective the score belongs to
        objective_name: Chat<'a>,
    },
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ScoreboardType {
    Integer,
    Hearts,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum ScoreboardPosition {
    List,
    Sidebar,
    BelowName,
    BlackTeam,
    DarkBlueTeam,
    DarkGreenTeam,
    DarkCyanTeam,
    DarkRedTeam,
    PurpleTeam,
    GoldTeam,
    GrayTeam,
    DarkGrayTeam,
    BlueTeam,
    GreenTeam,
    CyanTeam,
    RedTeam,
    PinkTeam,
    YellowTeam,
    WhiteTeam,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
#[discriminant(u8)]
pub enum TeamAction<'a> {
    Create {
        team_display_name: Chat<'a>,
        /// Bit mask. 0x01: Allow friendly fire, 0x02: can see invisible players on same team.
        friendly_flags: u8,
        /// One of the following: always, hideForOtherTeams, hideForOwnTeam, never
        name_tag_visibility: &'a str,
        /// One of the following: always, pushOtherTeams, pushOwnTeam, never
        collision_rule: &'a str,
        /// Used to color the name of players on the team
        team_color: TeamColor,
        /// Displayed before the names of players that are part of this team
        team_prefix: Chat<'a>,
        /// Displayed after the names of players that are part of this team
        team_suffix: Chat<'a>,
        /// Identifiers for the entities in this team. For players, this is their username; for other entities, it is their UUID.
        entities: Array<'a, &'a str, VarInt>,
    },
    Remove,
    Update {
        team_display_name: Chat<'a>,
        /// Bit mask. 0x01: Allow friendly fire, 0x02: can see invisible players on same team.
        friendly_flags: u8,
        /// One of the following: always, hideForOtherTeams, hideForOwnTeam, never
        name_tag_visibility: &'a str,
        /// One of the following: always, pushOtherTeams, pushOwnTeam, never
        collision_rule: &'a str,
        /// Used to color the name of players on the team
        team_color: TeamColor,
        /// Displayed before the names of players that are part of this team
        team_prefix: Chat<'a>,
        /// Displayed after the names of players that are part of this team
        team_suffix: Chat<'a>,
    },
    AddEntities {
        /// Identifiers for the added entities. For players, this is their username; for other entities, it is their UUID.
        entities: Array<'a, &'a str, VarInt>,
    },
    RemoveEntities {
        /// Identifiers for the removed entities. For players, this is their username; for other entities, it is their UUID.
        entities: Array<'a, &'a str, VarInt>,
    },
}

/// The color of a team defines how the names of the team members are visualized; any formatting code can be used.
#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum TeamColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkCyan,
    DarkRed,
    Purple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Cyan,
    Red,
    Pink,
    Yellow,
    White,
    Obfuscated,
    Bold,
    Strikethrough,
    Underlined,
    Italic,
    Reset,
}
