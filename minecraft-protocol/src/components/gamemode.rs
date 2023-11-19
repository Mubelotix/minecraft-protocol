use crate::*;

#[derive(PartialEq)]
#[minecraft_enum(u8)]
#[derive(Debug, Clone)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(i8)]
#[derive(Debug)]
pub enum PreviousGamemode {
    None = -1,

    Survival = 0,
    Creative,
    Adventure,
    Spectator,
}
