use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum GameState {
    /// Sends "You have no home bed or charged respawn anchor, or it was obstructed" to the client
    NoRespawn,
    BeginRaining,
    EndRaining,
    ChangeGamemode,
    Win,
    DemoEvent,
    ArrowHitPlayer,
    RainLevelChange,
    ThunderLevelChange,
    PlayPufferfishStingSound,
    ElderGuardianMob,
    EnableRespawnScreen,
    LimitedCrafting
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ClientStatus {
    /// Sent when the client is ready to complete login and when the client is ready to respawn after death
    PerformRespawn,
    /// Sent when the client opens the Statistics menu
    RequestStats,
}
