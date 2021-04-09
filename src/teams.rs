use crate::*;

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

#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ScoreboardType {
    Integer,
    Hearts
}

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
