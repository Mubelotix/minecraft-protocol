use std::convert::{TryFrom, TryInto};

use super::*;

pub trait MinecraftPacketPart<'a>: Sized {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    fn deserialize_minecraft_packet_part(input: &'a [u8]) -> Result<(Self, &'a [u8]), &'static str>;

    fn serialize_minecraft_packet(self) -> Result<Vec<u8>, &'static str> {
        let mut buffer = Vec::new();
        self.serialize_minecraft_packet_part(&mut buffer)?;
        Ok(buffer)
    }

    fn deserialize_uncompressed_minecraft_packet(input: &'a [u8]) -> Result<Self, &'static str> {
        let (result, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        if !input.is_empty() {
            return Err("There are still unparsed bytes after parsing.");
        }
        Ok(result)
    }

    fn deserialize_n(mut input: &'a [u8], n: usize) -> Result<(Vec<Self>, &'a [u8]), &'static str> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            let (item, new_input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
            input = new_input;
            result.push(item);
        }
        Ok((result, input))
    }
}

mod integers {
    use super::*;

    impl<'a> MinecraftPacketPart<'a> for bool {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self as u8);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            let (value, input) = input
                .split_first()
                .ok_or("Missing byte while parsing bool.")?;
            Ok((*value != 0, input))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i8 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self.to_le_bytes()[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            let (value, input) = input
                .split_first()
                .ok_or("Missing byte while parsing i8.")?;
            Ok((i8::from_le_bytes([*value]), input))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u8 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            let (value, input) = input
                .split_first()
                .ok_or("Missing byte while parsing u8.")?;
            Ok((*value, input))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for (i8, i8, i8) {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self.0.to_le_bytes()[0]);
            output.push(self.1.to_le_bytes()[0]);
            output.push(self.2.to_le_bytes()[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            let (first_byte, input) = input
                .split_first()
                .ok_or("Missing byte while parsing (i8, i8, i8).")?;
            let (second_byte, input) = input
                .split_first()
                .ok_or("Missing byte while parsing (i8, i8, i8).")?;
            let (third_byte, input) = input
                .split_first()
                .ok_or("Missing byte while parsing (i8, i8, i8).")?;
            Ok((
                (
                    i8::from_be_bytes([*first_byte]),
                    i8::from_be_bytes([*second_byte]),
                    i8::from_be_bytes([*third_byte]),
                ),
                input,
            ))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i16 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 2 {
                return Err("Missing byte while parsing i16.");
            }
            Ok(unsafe {
                (
                    i16::from_be_bytes(*(input.as_ptr() as *mut [u8; 2])),
                    input.get_unchecked(2..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u16 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 2 {
                return Err("Missing byte while parsing u16.");
            }
            Ok(unsafe {
                (
                    u16::from_be_bytes(*(input.as_ptr() as *mut [u8; 2])),
                    input.get_unchecked(2..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i32 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 4 {
                return Err("Missing byte while parsing i32.");
            }
            Ok(unsafe {
                (
                    i32::from_be_bytes(*(input.as_ptr() as *mut [u8; 4])),
                    input.get_unchecked(4..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u32 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 4 {
                return Err("Missing byte while parsing u32.");
            }
            Ok(unsafe {
                (
                    u32::from_be_bytes(*(input.as_ptr() as *mut [u8; 4])),
                    input.get_unchecked(4..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i64 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[7]);
            output.push(bytes[6]);
            output.push(bytes[5]);
            output.push(bytes[4]);
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 8 {
                return Err("Missing byte while parsing i64.");
            }
            Ok(unsafe {
                (
                    i64::from_be_bytes(*(input.as_ptr() as *mut [u8; 8])),
                    input.get_unchecked(8..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u64 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[7]);
            output.push(bytes[6]);
            output.push(bytes[5]);
            output.push(bytes[4]);
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 8 {
                return Err("Missing byte while parsing i64.");
            }
            Ok(unsafe {
                (
                    u64::from_be_bytes(*(input.as_ptr() as *mut [u8; 8])),
                    input.get_unchecked(8..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u128 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[15]);
            output.push(bytes[14]);
            output.push(bytes[13]);
            output.push(bytes[12]);
            output.push(bytes[11]);
            output.push(bytes[10]);
            output.push(bytes[9]);
            output.push(bytes[8]);
            output.push(bytes[7]);
            output.push(bytes[6]);
            output.push(bytes[5]);
            output.push(bytes[4]);
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 16 {
                return Err("Missing byte while parsing u128 (UUID).");
            }
            Ok(unsafe {
                (
                    u128::from_be_bytes(*(input.as_ptr() as *mut [u8; 16])),
                    input.get_unchecked(16..),
                )
            })
        }
    }

    impl<'a> MinecraftPacketPart<'a> for f32 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 4 {
                return Err("Missing byte while parsing f32.");
            }
            unsafe {
                let number = input.get_unchecked(..4);
                #[cfg(target_endian = "little")]
                let number = f32::from_be_bytes([
                    *number.get_unchecked(0),
                    *number.get_unchecked(1),
                    *number.get_unchecked(2),
                    *number.get_unchecked(3),
                ]);

                Ok((number, input.get_unchecked(4..)))
            }
        }
    }

    impl<'a> MinecraftPacketPart<'a> for f64 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[7]);
            output.push(bytes[6]);
            output.push(bytes[5]);
            output.push(bytes[4]);
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(Self, &[u8]), &'static str> {
            if input.len() < 8 {
                return Err("Missing byte while parsing f64.");
            }
            unsafe {
                let number = input.get_unchecked(..8);
                #[cfg(target_endian = "little")]
                let number = f64::from_be_bytes([
                    *number.get_unchecked(0),
                    *number.get_unchecked(1),
                    *number.get_unchecked(2),
                    *number.get_unchecked(3),
                    *number.get_unchecked(4),
                    *number.get_unchecked(5),
                    *number.get_unchecked(6),
                    *number.get_unchecked(7),
                ]);

                Ok((number, input.get_unchecked(8..)))
            }
        }
    }

    impl<'a> MinecraftPacketPart<'a> for VarInt {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let mut value: u32 = unsafe { std::mem::transmute(self.0) };

            loop {
                let mut temp = (value & 0b01111111) as u8;
                value >>= 7;

                if value == 0 {
                    output.push(temp);
                    return Ok(());
                } else {
                    temp += 0b10000000;
                    output.push(temp);
                }
            }
        }

        fn deserialize_minecraft_packet_part(
            mut input: &[u8],
        ) -> Result<(Self, &[u8]), &'static str> {
            let mut result: u32 = 0;
            let mut num_read: u32 = 0;

            loop {
                let (read, new_input) =
                    input.split_first().ok_or("Not enough bytes for varint!")?;
                let read = *read;
                input = new_input;
                let mut value: u32 = (read & 0b01111111) as u32;
                if num_read == 5 {
                    value &= 0b1111;
                    result |= value << (4 * num_read);
                } else {
                    result |= value << (7 * num_read);
                }

                num_read += 1;
                if num_read > 5 {
                    return Err("VarInt is too big");
                }

                if read & 0b10000000 == 0 {
                    let result: i32 = unsafe { std::mem::transmute(result) };
                    return Ok((VarInt(result), input));
                }
            }
        }
    }

    impl<'a> MinecraftPacketPart<'a> for VarLong {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let mut value: u64 = unsafe { std::mem::transmute(self.0) };

            loop {
                let mut temp = (value & 0b01111111) as u8;
                value >>= 7;

                if value == 0 {
                    output.push(temp);
                    return Ok(());
                } else {
                    temp += 0b10000000;
                    output.push(temp);
                }
            }
        }

        fn deserialize_minecraft_packet_part(
            mut input: &[u8],
        ) -> Result<(Self, &[u8]), &'static str> {
            let mut result: u64 = 0;
            let mut num_read: u64 = 0;

            loop {
                let (read, new_input) = input.split_first().unwrap();
                let read = *read;
                input = new_input;
                let value = (read & 0b01111111) as u64;
                result |= value << (7 * num_read);

                num_read += 1;
                if num_read > 10 {
                    return Err("VarLong is too big");
                }

                if read & 0b10000000 == 0 {
                    let result: i64 = unsafe { std::mem::transmute(result) };
                    return Ok((VarLong(result), input));
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;

        #[test]
        fn test_varint_serialization() {
            let inputs = [0, 1, 2, 127, 128, 255, 2097151, 2147483647, -1, -2147483648];
            let expected_outputs: [&[u8]; 10] = [
                &[0],
                &[1],
                &[2],
                &[127],
                &[128, 1],
                &[255, 1],
                &[255, 255, 127],
                &[255, 255, 255, 255, 7],
                &[255, 255, 255, 255, 15],
                &[128, 128, 128, 128, 8],
            ];
            let mut outputs = Vec::new();

            for input in &inputs {
                let mut buffer = Vec::new();
                VarInt(*input)
                    .serialize_minecraft_packet_part(&mut buffer)
                    .unwrap();
                outputs.push(buffer);
            }

            for idx in 0..inputs.len() {
                assert_eq!(outputs[idx], expected_outputs[idx]);
            }
        }

        #[test]
        fn test_varint_deserialization() {
            let expected_outputs = [0, 1, 2, 127, 128, 255, 2097151, 2147483647, -1, -2147483648];
            let inputs: [&[u8]; 10] = [
                &mut [0],
                &mut [1],
                &mut [2],
                &mut [127],
                &mut [128, 1],
                &mut [255, 1],
                &mut [255, 255, 127],
                &mut [255, 255, 255, 255, 7],
                &mut [255, 255, 255, 255, 15],
                &mut [128, 128, 128, 128, 8],
            ];
            let mut outputs = Vec::new();

            #[allow(clippy::needless_range_loop)]
            for idx in 0..inputs.len() {
                let (result, _) = VarInt::deserialize_minecraft_packet_part(inputs[idx]).unwrap();
                outputs.push(result.0);
            }

            for idx in 0..inputs.len() {
                assert_eq!(outputs[idx], expected_outputs[idx]);
            }
        }

        #[test]
        fn test_varlong_serialization() {
            let inputs = [
                0,
                1,
                2,
                127,
                128,
                255,
                2147483647,
                9223372036854775807,
                -1,
                -2147483648,
                -9223372036854775808,
            ];
            let expected_outputs: [&[u8]; 11] = [
                &[0],
                &[1],
                &[2],
                &[127],
                &[128, 1],
                &[255, 1],
                &[255, 255, 255, 255, 7],
                &[255, 255, 255, 255, 255, 255, 255, 255, 127],
                &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1],
                &[128, 128, 128, 128, 248, 255, 255, 255, 255, 1],
                &[128, 128, 128, 128, 128, 128, 128, 128, 128, 1],
            ];
            let mut outputs = Vec::new();

            for input in &inputs {
                let mut buffer = Vec::new();
                VarLong(*input)
                    .serialize_minecraft_packet_part(&mut buffer)
                    .unwrap();
                outputs.push(buffer);
            }

            for idx in 0..inputs.len() {
                assert_eq!(outputs[idx], expected_outputs[idx]);
            }
        }

        #[test]
        fn test_varlong_deserialization() {
            let expected_outputs = [
                0,
                1,
                2,
                127,
                128,
                255,
                2147483647,
                9223372036854775807,
                -1,
                -2147483648,
                -9223372036854775808,
            ];
            let inputs: [&[u8]; 11] = [
                &mut [0],
                &mut [1],
                &mut [2],
                &mut [127],
                &mut [128, 1],
                &mut [255, 1],
                &mut [255, 255, 255, 255, 7],
                &mut [255, 255, 255, 255, 255, 255, 255, 255, 127],
                &mut [255, 255, 255, 255, 255, 255, 255, 255, 255, 1],
                &mut [128, 128, 128, 128, 248, 255, 255, 255, 255, 1],
                &mut [128, 128, 128, 128, 128, 128, 128, 128, 128, 1],
            ];
            let mut outputs = Vec::new();

            #[allow(clippy::needless_range_loop)]
            for idx in 0..inputs.len() {
                let (result, _) = VarLong::deserialize_minecraft_packet_part(inputs[idx]).unwrap();
                outputs.push(result.0);
            }

            for idx in 0..inputs.len() {
                assert_eq!(outputs[idx], expected_outputs[idx]);
            }
        }

        #[test]
        fn test_position() {
            let position = Position {
                x: 10,
                y: 65,
                z: 23,
            };
            let mut serialized = position.clone().serialize_minecraft_packet().unwrap();
            let deserialized =
                Position::deserialize_uncompressed_minecraft_packet(serialized.as_mut_slice())
                    .unwrap();
            assert_eq!(position, deserialized);

            let position = Position {
                x: -122,
                y: 65,
                z: 23,
            };
            let mut serialized = position.clone().serialize_minecraft_packet().unwrap();
            let deserialized =
                Position::deserialize_uncompressed_minecraft_packet(serialized.as_mut_slice())
                    .unwrap();
            assert_eq!(position, deserialized);

            let position = Position { x: 0, y: 65, z: 23 };
            let mut serialized = position.clone().serialize_minecraft_packet().unwrap();
            let deserialized =
                Position::deserialize_uncompressed_minecraft_packet(serialized.as_mut_slice())
                    .unwrap();
            assert_eq!(position, deserialized);

            let position = Position {
                x: 10,
                y: -20,
                z: 23,
            };
            let mut serialized = position.clone().serialize_minecraft_packet().unwrap();
            let deserialized =
                Position::deserialize_uncompressed_minecraft_packet(serialized.as_mut_slice())
                    .unwrap();
            assert_eq!(position, deserialized);

            let position = Position {
                x: -941621,
                y: -846,
                z: -6546541,
            };
            let mut serialized = position.clone().serialize_minecraft_packet().unwrap();
            let deserialized =
                Position::deserialize_uncompressed_minecraft_packet(serialized.as_mut_slice())
                    .unwrap();
            assert_eq!(position, deserialized);
        }
    }
}

impl<'a> MinecraftPacketPart<'a> for &'a str {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len = VarInt(self.len().try_into().map_err(|_| "String too long")?);
        len.serialize_minecraft_packet_part(output)?;
        output.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(&str, &[u8]), &'static str> {
        let (len, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        if len.0 <= 0 {
            return Ok(("", input));
        }
        let len: usize = len.0 as usize;
        if len > input.len() {
            return Err("String claims ownership of too much data");
        }
        let (slice, input) = input.split_at(len);
        let string = std::str::from_utf8(slice).map_err(|_| "Invalid utf8")?;

        Ok((string, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for String {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len = VarInt(self.len().try_into().map_err(|_| "String too long")?);
        len.serialize_minecraft_packet_part(output)?;
        output.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn deserialize_minecraft_packet_part(input: &[u8]) -> Result<(String, &[u8]), &'static str> {
        let (len, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        if len.0 <= 0 {
            return Ok((String::new(), input));
        }
        let len: usize = len.0 as usize;
        if len > input.len() {
            return Err("String claims ownership of too much data");
        }
        let (slice, input) = input.split_at(len);
        let string = String::from_utf8(slice.to_vec()).map_err(|_| "Invalid utf8")?;

        Ok((string, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for Position {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let x = match self.x < 0 {
            true => (self.x + 2i32.pow(26)) as u64,
            false => self.x as u64,
        };
        let y = match self.y < 0 {
            true => (self.y + 2i16.pow(12)) as u64,
            false => self.y as u64,
        };
        let z = match self.z < 0 {
            true => (self.z + 2i32.pow(26)) as u64,
            false => self.z as u64,
        };
        let value = ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF);
        output.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        if input.len() < 8 {
            return Err("Missing bytes in position");
        }
        let (bytes, input) = input.split_at(8);
        let total = unsafe {
            u64::from_le_bytes([
                *bytes.get_unchecked(7),
                *bytes.get_unchecked(6),
                *bytes.get_unchecked(5),
                *bytes.get_unchecked(4),
                *bytes.get_unchecked(3),
                *bytes.get_unchecked(2),
                *bytes.get_unchecked(1),
                *bytes.get_unchecked(0),
            ])
        };

        let mut x = (total >> 38) as i32 & 0b11_1111_1111_1111_1111_1111_1111;
        if x >= 2i32.pow(25) {
            x -= 2i32.pow(26)
        }
        let mut y = (total & 0xFFF) as i16;
        if y >= 2i16.pow(11) {
            y -= 2i16.pow(12);
        }
        let mut z = (total << 26 >> 38) as i32;
        if z >= 2i32.pow(25) {
            z -= 2i32.pow(26)
        }

        Ok((Position { x, y, z }, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for RawBytes<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.extend_from_slice(self.data);
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let data = input;
        Ok((RawBytes { data }, &mut []))
    }
}

impl<
        'a,
        T: MinecraftPacketPart<'a> + std::fmt::Debug,
        U: MinecraftPacketPart<'a> + TryFrom<usize> + TryInto<usize>,
    > MinecraftPacketPart<'a> for Array<'a, T, U>
{
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len: U = U::try_from(self.items.len())
            .map_err(|_| "The array lenght cannot be serialized due to its type.")?;
        len.serialize_minecraft_packet_part(output)?;
        for item in self.items {
            item.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (len, mut input) = U::deserialize_minecraft_packet_part(input)?;
        let len: usize = len
            .try_into()
            .map_err(|_| "The array lenght cannot be deserialized due to its type.")?;
        let mut items = Vec::new();
        for _ in 0..len {
            let (item, new_input) = T::deserialize_minecraft_packet_part(input)?;
            items.push(item);
            input = new_input;
        }
        Ok((
            Array {
                items,
                _len_prefix: std::marker::PhantomData,
            },
            input,
        ))
    }
}


impl<
    'a,
    const N: usize,
    > MinecraftPacketPart<'a> for [u8; N] {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.extend_from_slice(&self);
        Ok(())
    }

    fn deserialize_minecraft_packet_part(input: &'a [u8]) -> Result<(Self, &'a [u8]), &'static str> {
        if input.len() < N {
            return Err("Not enough data to deserialize");
        }

        let (data, rest) = input.split_at(N);
        // TODO: not copy the data
        Ok((data.try_into().map_err(|_| "Impossible to copy the slice")?, rest))
    }
}

pub struct FixedSizeArray<'a, V: MinecraftPacketPart<'a>, const N: usize> {
    pub items: Vec<V>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, V: MinecraftPacketPart<'a>, const N: usize> MinecraftPacketPart<'a> for FixedSizeArray<'a, V, N> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        if self.items.len() != N {
            return Err("The vector length is not the expected one");
        }
        for item in self.items {
            item.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }

    fn deserialize_minecraft_packet_part(mut input: &'a [u8]) -> Result<(Self, &'a [u8]), &'static str> {
        let mut items = Vec::new();
        for _ in 0..N {
            let (item, new_input) = V::deserialize_minecraft_packet_part(input)?;
            items.push(item);
            input = new_input;
        }
        Ok((FixedSizeArray { items, _phantom: std::marker::PhantomData }, input))
    }
}


impl<
        'a,
        K: MinecraftPacketPart<'a> + std::fmt::Debug + std::cmp::Ord,
        V: MinecraftPacketPart<'a> + std::fmt::Debug,
        U: MinecraftPacketPart<'a> + TryFrom<usize> + TryInto<usize>,
    > MinecraftPacketPart<'a> for Map<'a, K, V, U>
{
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len = U::try_from(self.items.len())
            .map_err(|_| "The map lenght cannot be serialized due to its type.")?;
        len.serialize_minecraft_packet_part(output)?;
        for (key, value) in self.items.into_iter() {
            key.serialize_minecraft_packet_part(output)?;
            value.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let mut items = std::collections::BTreeMap::new();
        let (len, mut input) = U::deserialize_minecraft_packet_part(input)?;
        let len: usize = len
            .try_into()
            .map_err(|_| "The map lenght cannot be deserialized due to its type.")?;

        for _ in 0..len {
            let (key, new_input) = K::deserialize_minecraft_packet_part(input)?;
            let (value, new_input) = V::deserialize_minecraft_packet_part(new_input)?;
            input = new_input;
            items.insert(key, value);
        }

        Ok((
            Map {
                items,
                _len_prefix: std::marker::PhantomData,
            },
            input,
        ))
    }
}

impl<'a, T: MinecraftPacketPart<'a>> MinecraftPacketPart<'a> for Option<T> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        if let Some(value) = self {
            true.serialize_minecraft_packet_part(output)?;
            value.serialize_minecraft_packet_part(output)?;
        } else {
            false.serialize_minecraft_packet_part(output)?;
        }
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a [u8],
    ) -> Result<(Self, &'a [u8]), &'static str> {
        let (is_some, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
        if is_some {
            let (value, input) = MinecraftPacketPart::deserialize_minecraft_packet_part(input)?;
            Ok((Some(value), input))
        } else {
            Ok((None, input))
        }
    }
}

pub type BitSet<'a> = Array<'a, i64, VarInt>;


#[test]
fn print_varint() {
    let value = 0x25;
    let varint = VarInt::from(value);
    let mut data = Vec::new();
    varint.serialize_minecraft_packet_part(&mut data).unwrap();
    println!("{:?}", data);
}
