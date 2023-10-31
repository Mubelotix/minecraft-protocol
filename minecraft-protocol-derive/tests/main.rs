extern crate minecraft_packet_derive;
use minecraft_packet_derive::*;

#[derive(Debug, MinecraftPacketPart, PartialEq, Clone)]
pub struct Test<'a> {
    data: u8,
    other: &'a str,
}

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

#[test]
fn main() {
    let data = Test {
        data: 5,
        other: "sfd",
    };
    let mut serialized = data.clone().serialize_minecraft_packet().unwrap();
    let deserialized = Test::deserialize_minecraft_packet(&mut serialized).unwrap();
    assert_eq!(data, deserialized);
}
