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

#[derive(Debug, MinecraftPacketPart)]
pub enum TitleAction<'a> {
    SetTitle {
        title: Chat<'a>,
    },
    SetSubtitle {
        subtitle: Chat<'a>,
    },
    SetActionBar {
        /// Displays a message above the hotbar (the same as [Position::GameInfo] in [ClientBoundPacket::ChatMessage], except that it correctly renders formatted chat; see MC-119145 for more information).
        action_bar_text: Chat<'a>,
    },
    SetTimes {
        /// Ticks to spend fading in
        fade_int: i32,
        /// Ticks to keep the title displayed
        stay: i32,
        /// Ticks to spend out, not when to start fading out
        fade_out: i32,
    },
    /// Sending [TitleAction::Hide] once makes the text disappear.
    /// Sending another time will make the text reappear.
    Hide,
    /// Erases the text
    Reset,
}
