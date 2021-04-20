use super::NbtTag;
use crate::*;

impl<'a> MinecraftPacketPart<'a> for NbtTag {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        todo!()
    }

    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        super::parse_nbt(input)
    }
}
