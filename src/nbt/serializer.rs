use super::NbtTag;
use crate::*;

impl<'a> MinecraftPacketPart<'a> for NbtTag {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.serialize(output);
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        super::parse_network_nbt(input)
    }
}
