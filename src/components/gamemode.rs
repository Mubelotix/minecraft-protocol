use crate::*;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[minecraft_enum(i8)]
#[derive(Debug)]
pub enum PreviousGamemode {
    None = -1,

    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}
