use crate::*;

#[derive(Debug)]
/// One eligible value to insert
pub struct Match<'a> {
    /// The value. Note that for instance this doesn't include a leading / on commands.
    pub value: &'a str,
    /// Tooltip to display
    pub tooltip: Option<crate::packets::Chat<'a>>,
}

impl<'a> MinecraftPacketPart<'a> for Match<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.value.serialize_minecraft_packet_part(output)?;
        if let Some(tooltip) = self.tooltip {
            true.serialize_minecraft_packet_part(output)?;
            tooltip.serialize_minecraft_packet_part(output)?;
        } else {
            false.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }
    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (value, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (has_tooltip, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        if has_tooltip {
            let (tooltip, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
            Ok((Match { value, tooltip: Some(tooltip) }, input))
        } else {
            Ok((Match { value, tooltip: None }, input))
        }
    }
}
