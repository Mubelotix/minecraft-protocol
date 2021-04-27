use crate::*;

#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum CommandBlockMode {
    Sequence,
    Auto,
    Redstone,
}
