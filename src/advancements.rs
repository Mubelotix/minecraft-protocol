use crate::*;

#[derive(Debug, MinecraftPacketPart)]
pub struct Advancement<'a> {
    /// The identifier of the parent advancement
    pub parent_id: Option<Identifier<'a>>,
    pub display_data: Option<AdvancementDisplay<'a>>,
    pub criteria: Array<'a, Identifier<'a>, VarInt>,
    /// Array of arrays of required criteria
    pub requirements: Array<'a, Array<'a, &'a str, VarInt>, VarInt>,
}

#[derive(Debug)]
pub struct AdvancementDisplay<'a> {
    pub title: Chat<'a>,
    pub description: Chat<'a>,
    pub icon: crate::slots::Slot<'a>,
    pub frame_type: AdvancementFrameType,
    pub show_toast: bool,
    pub hidden: bool,
    pub background_texture: Option<Identifier<'a>>,
    pub x: f32,
    pub y: f32,
}

/// A map linking criterion identifiers to their progress.
pub type AdvancementProgress<'a> = Map<'a, Identifier<'a>, CriterionProgress, VarInt>;

/// Contains the date of achieving or `None` if it has not been achieved.
pub type CriterionProgress = Option<i64>;

impl<'a> MinecraftPacketPart<'a> for AdvancementDisplay<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        self.title.serialize_minecraft_packet_part(output)?;
        self.description.serialize_minecraft_packet_part(output)?;
        self.icon.serialize_minecraft_packet_part(output)?;
        self.frame_type.serialize_minecraft_packet_part(output)?;
        let flags = (self.background_texture.is_some() as u8)
            + ((self.show_toast as u8) << 1)
            + ((self.hidden as u8) << 2);
        flags.serialize_minecraft_packet_part(output)?;
        if let Some(background_texture) = self.background_texture {
            background_texture.serialize_minecraft_packet_part(output)?;
        }
        self.x.serialize_minecraft_packet_part(output)?;
        self.y.serialize_minecraft_packet_part(output)?;
        Ok(())
    }
    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (title, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (description, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (icon, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (frame_type, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (flags, input) = u8::deserialize_minecraft_packet_part(input)?;
        let has_background_texture = flags & 0b0000_0001 != 0;
        let show_toast = flags & 0b0000_0010 != 0;
        let hidden = flags & 0b0000_0100 != 0;
        let (background_texture, input) = match has_background_texture {
            true => {
                let (background_texture, input) =
                    MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
                (Some(background_texture), input)
            }
            false => (None, input),
        };
        let (x, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        let (y, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        Ok((
            AdvancementDisplay {
                title,
                description,
                icon,
                frame_type,
                show_toast,
                hidden,
                background_texture,
                x,
                y,
            },
            input,
        ))
    }
}

#[minecraft_enum(VarInt)]
#[derive(Debug)]
pub enum AdvancementFrameType {
    Task,
    Challenge,
    Goal,
}
