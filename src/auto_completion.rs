use crate::*;

/// One eligible value to insert
#[derive(Debug, MinecraftPacketPart)]
pub struct Match<'a> {
    /// The value. Note that for instance this doesn't include a leading / on commands.
    pub value: &'a str,
    /// Tooltip to display
    pub tooltip: Option<crate::packets::Chat<'a>>,
}
