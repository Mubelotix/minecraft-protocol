use crate::{*, packets::serializer::BitSet};

/// See [processing chat](https://wiki.vg/Chat#Processing_chat) for more information
#[cfg_attr(test, derive(PartialEq))]
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
#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

#[cfg_attr(test, derive(PartialEq))]
#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum ChatAction {
    Add,
    Remove,
    Set
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
pub struct PreviousMessage<'a> {
    /// The message Id + 1, used for validating message signature. The next field is present only when value of this field is equal to 0.
    pub message_id: VarInt,
    /// The previous message's signature. Contains the same type of data as Message Signature bytes (256 bytes) above. Not length-prefxied.
    pub signature: RawBytes<'a>,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum FilterType<'a> {
    /// No filters applied
    PassThrough,
    /// All filters applied
    FullyFiltered,
    /// Only some filters are applied.
    PartiallyFiltered {
        filter_mask: BitSet<'a>,
    },
}
