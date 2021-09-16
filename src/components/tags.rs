use crate::*;

/// A tag
/// 
/// More information on tags is available at: https://minecraft.gamepedia.com/Tag
/// And a list of all tags is here: https://minecraft.gamepedia.com/Tag#List_of_tags
#[derive(Debug, MinecraftPacketPart)]
pub struct Tag<'a> {
    pub tag_name: Identifier<'a>,
    pub data: Array<'a, VarInt, VarInt>,
}
