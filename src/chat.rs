use crate::*;

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Position {
    /// A common chat (chat box)
    Chat,
    /// A system message (chat box)
    System,
    /// Game info displayed above the hotbar
    GameInfo
}
