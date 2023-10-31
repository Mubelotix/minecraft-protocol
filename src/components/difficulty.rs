use crate::*;

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Normal,
    Hard,
}
