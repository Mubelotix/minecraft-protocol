use crate::*;
use super::NbtTag;

impl<'a> MinecraftPacketPart<'a> for NbtTag<'a> {
    fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        todo!()
    }

    fn build_from_minecraft_packet(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        super::parse_nbt(input)
    }
}