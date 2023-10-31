extern crate minecraft_packet_derive;
use minecraft_packet_derive::*;

pub trait MinecraftPacketPart<'a>: Sized {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str>;

    fn serialize_minecraft_packet(self) -> Result<Vec<u8>, &'static str> {
        let mut buffer = Vec::new();
        self.serialize_minecraft_packet_part(&mut buffer)?;
        Ok(buffer)
    }

    fn deserialize_minecraft_packet(input: &'a mut [u8]) -> Result<Self, &'static str> {
        let (result, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        if !input.is_empty() {
            return Err("There are still unparsed bytes after parsing.");
        }
        Ok(result)
    }
}

impl<'a> MinecraftPacketPart<'a> for u8 {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.push(self);
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &mut [u8],
    ) -> Result<(Self, &mut [u8]), &'static str> {
        let (value, input) = input.split_first_mut().unwrap();
        Ok((*value, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for &'a str {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.push(self.len() as u8);
        output.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let (len, input) = input.split_first_mut().unwrap();
        let (slice, input) = input.split_at_mut(*len as usize);
        Ok((std::str::from_utf8(slice).unwrap(), input))
    }
}

#[derive(MinecraftPacketPart)]
#[discriminant(u8)]
pub enum TestEnum<'a> {
    Teacher {
        student_count: u8,
        grade_average: u8,
    },
    #[value = 5]
    Farmer {
        root_meters_count: u8,
        name: &'a str,
    },
    NoOne,
}
