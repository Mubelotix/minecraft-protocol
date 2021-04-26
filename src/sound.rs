use crate::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum SoundCategory {
    Master,
    Music,
    Record,
    Weather,
    Block,
    Hostile,
    Neutral,
    Player,
    Ambiant,
    Voice,
}

#[derive(Debug)]
pub struct StopSoundPacket<'a> {
    /// If not present, then sounds from all sources are cleared
    pub sound_category: Option<SoundCategory>,
    /// A sound effect name, see [ClientboundPacket::NamedSoundEffect].
    /// If not present, then all sounds are cleared.
    pub sound_effect_name: Option<Identifier<'a>>,
}

impl<'a> MinecraftPacketPart<'a> for StopSoundPacket<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let mut flags: u8 = 0;
        if self.sound_category.is_some() {
            flags += 0b0000_0001;
        }
        if self.sound_category.is_some() {
            flags += 0b0000_0010;
        }
        output.push(flags);
        if let Some(sound_category) = self.sound_category {
            sound_category.serialize_minecraft_packet_part(output)?;
        }
        if let Some(sound_effect_name) = self.sound_effect_name {
            sound_effect_name.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }
    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (flags, input) = u8::deserialize_minecraft_packet_part(input)?;
        let (sound_category, input) = match flags & 0b0000_0001 == 1 {
            true => {
                let (sound_category, input) =
                    MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
                (Some(sound_category), input)
            }
            false => (None, input),
        };
        let (sound_effect_name, input) = match flags & 0b0000_0010 == 0b0000_0010 {
            true => {
                let (sound_effect_name, input) =
                    MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
                (Some(sound_effect_name), input)
            }
            false => (None, input),
        };
        Ok((
            StopSoundPacket {
                sound_category,
                sound_effect_name,
            },
            input,
        ))
    }
}
