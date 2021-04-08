use crate::*;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum GameState {
    /// Sends "You have no home bed or charged respawn anchor, or it was obstructed" to the client
    NoRespawn,
    EndRaining,
    BeginRaining,
    ChangeGamemode,
    Win,
    DemoEvent,
    ArrowHit,
    RainLevelChange,
    ThunderLevelChange,
    PlayPufferfishStingSound,
    ElderGuardianMob,
    EnableRespawn,
}
