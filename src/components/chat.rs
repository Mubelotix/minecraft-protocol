use crate::*;

/// See [processing chat](https://wiki.vg/Chat#Processing_chat) for more information
#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Position {
    /// A common chat (chat box)
    Chat,
    /// A system message (chat box)
    System,
    /// Game info displayed above the hotbar
    GameInfo,
}

/// See [processing chat](https://wiki.vg/Chat#Processing_chat) for more information
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}
