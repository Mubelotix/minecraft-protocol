use crate::packets::serializer::MinecraftPacketPart;

#[derive(Debug)]
#[repr(u8)]
pub enum Animation {
    SwingMainArm = 0,
    TakeDamage,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect = 5,
}

impl<'a> MinecraftPacketPart<'a> for Animation {
    fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.push(self as u8);
        Ok(())
    }

    fn build_from_minecraft_packet(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (animation_id, input) = u8::build_from_minecraft_packet(input)?;
        let animation = match animation_id {
            0 => Animation::SwingMainArm,
            1 => Animation::TakeDamage,
            2 => Animation::LeaveBed,
            3 => Animation::SwingOffhand,
            4 => Animation::CriticalEffect,
            5 => Animation::MagicCriticalEffect,
            _ => return Err("The animation ID is outside the definition range"),
        };
        Ok((animation, input))
    }
}
