use std::convert::TryInto;

use super::*;

pub trait MinecraftPacketPart<'a>: Sized {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    fn deserialize_minecraft_packet_part(input: &'a mut [u8]) -> Result<(Self, &'a mut [u8]), &'static str>;

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

mod integers {
    use super::*;

    impl<'a> MinecraftPacketPart<'a> for bool {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self as u8);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing bool.")?;
            Ok((*value != 0, input))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i8 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self.to_le_bytes()[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing i8.")?;
            Ok((i8::from_le_bytes([*value]), input))
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
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing u8.")?;
            Ok((*value, input))
        }
    }

    impl<'a> MinecraftPacketPart<'a> for i16 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 2 {
                return Err("Missing byte while parsing i16.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 2),
                    std::slice::from_raw_parts_mut(ptr.add(2), len - 2),
                );

                Ok((
                    i16::from_le_bytes([*bytes.get_unchecked(1), *bytes.get_unchecked(0)]),
                    input,
                ))
            }
        }
    }

    impl<'a> MinecraftPacketPart<'a> for u16 {
        fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 2 {
                return Err("Missing byte while parsing u16.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 2),
                    std::slice::from_raw_parts_mut(ptr.add(2), len - 2),
                );

                Ok((
                    u16::from_le_bytes([*bytes.get_unchecked(1), *bytes.get_unchecked(0)]),
                    input,
                ))
            }
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

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 4 {
                return Err("Missing byte while parsing i32.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 4),
                    std::slice::from_raw_parts_mut(ptr.add(4), len - 4),
                );

                Ok((
                    i32::from_le_bytes([
                        *bytes.get_unchecked(3),
                        *bytes.get_unchecked(2),
                        *bytes.get_unchecked(1),
                        *bytes.get_unchecked(0),
                    ]),
                    input,
                ))
            }
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

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 8 {
                return Err("Missing byte while parsing i64.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 8),
                    std::slice::from_raw_parts_mut(ptr.add(8), len - 8),
                );

                Ok((
                    i64::from_le_bytes([
                        *bytes.get_unchecked(7),
                        *bytes.get_unchecked(6),
                        *bytes.get_unchecked(5),
                        *bytes.get_unchecked(4),
                        *bytes.get_unchecked(3),
                        *bytes.get_unchecked(2),
                        *bytes.get_unchecked(1),
                        *bytes.get_unchecked(0),
                    ]),
                    input,
                ))
            }
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

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 16 {
                return Err("Missing byte while parsing u128 (UUID).");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 16),
                    std::slice::from_raw_parts_mut(ptr.add(16), len - 16),
                );

                Ok((
                    u128::from_le_bytes([
                        *bytes.get_unchecked(15),
                        *bytes.get_unchecked(14),
                        *bytes.get_unchecked(13),
                        *bytes.get_unchecked(12),
                        *bytes.get_unchecked(11),
                        *bytes.get_unchecked(10),
                        *bytes.get_unchecked(9),
                        *bytes.get_unchecked(8),
                        *bytes.get_unchecked(7),
                        *bytes.get_unchecked(6),
                        *bytes.get_unchecked(5),
                        *bytes.get_unchecked(4),
                        *bytes.get_unchecked(3),
                        *bytes.get_unchecked(2),
                        *bytes.get_unchecked(1),
                        *bytes.get_unchecked(0),
                    ]),
                    input,
                ))
            }
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

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 4 {
                return Err("Missing byte while parsing i32.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 4),
                    std::slice::from_raw_parts_mut(ptr.add(4), len - 4),
                );

                Ok((
                    f32::from_le_bytes([
                        *bytes.get_unchecked(3),
                        *bytes.get_unchecked(2),
                        *bytes.get_unchecked(1),
                        *bytes.get_unchecked(0),
                    ]),
                    input,
                ))
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

        fn deserialize_minecraft_packet_part(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            if input.len() < 8 {
                return Err("Missing byte while parsing i64.");
            }
            unsafe {
                let len = input.len();
                let ptr = input.as_mut_ptr();
                let (bytes, input) = (
                    std::slice::from_raw_parts_mut(ptr, 8),
                    std::slice::from_raw_parts_mut(ptr.add(8), len - 8),
                );

                Ok((
                    f64::from_le_bytes([
                        *bytes.get_unchecked(7),
                        *bytes.get_unchecked(6),
                        *bytes.get_unchecked(5),
                        *bytes.get_unchecked(4),
                        *bytes.get_unchecked(3),
                        *bytes.get_unchecked(2),
                        *bytes.get_unchecked(1),
                        *bytes.get_unchecked(0),
                    ]),
                    input,
                ))
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
            mut input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let mut result: u32 = 0;
            let mut num_read: u32 = 0;

            loop {
                let (read, new_input) = input.split_first_mut().unwrap();
                let read = *read;
                input = new_input;
                let value: u32 = (read & 0b01111111) as u32;
                result |= value << (7 * num_read);

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
            mut input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let mut result: u64 = 0;
            let mut num_read: u64 = 0;

            loop {
                let (read, new_input) = input.split_first_mut().unwrap();
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
            let inputs: [&mut [u8]; 10] = [
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
            let inputs: [&mut [u8]; 11] = [
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
    }
}

impl<'a> MinecraftPacketPart<'a> for &'a str {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let len = VarInt(self.len().try_into().map_err(|_| "String too long")?);
        len.serialize_minecraft_packet_part(output)?;
        output.extend_from_slice(self.as_bytes());
        Ok(())
    }

    fn deserialize_minecraft_packet_part(input: &mut [u8]) -> Result<(&str, &mut [u8]), &'static str> {
        let (len, input) = VarInt::deserialize_minecraft_packet_part(input)?;
        if len.0 <= 0 {
            return Ok(("", input));
        }
        let len: usize = len.0 as usize;
        if len > input.len() {
            return Err("String claims ownership of too much data");
        }
        let (slice, input) = input.split_at_mut(len);
        let string = std::str::from_utf8(slice).map_err(|_| "Invalid utf8")?;

        Ok((string, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for Position {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        let mut total: u64 = (unsafe { std::mem::transmute::<i32, u32>(self.x) }
            & 0b00000011111111111111111111111111) as u64;
        total <<= 26;
        total += (unsafe { std::mem::transmute::<i16, u16>(self.y) } & 0b0000001111111111) as u64;
        total <<= 12;
        total += (unsafe { std::mem::transmute::<i32, u32>(self.z) }
            & 0b00000011111111111111111111111111) as u64;

        let bytes = total.to_le_bytes();
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

    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        if input.len() < 8 {
            return Err("Missing bytes in position");
        }
        let (bytes, input) = input.split_at_mut(8);
        let mut total = unsafe {
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

        let z: i32 =
            unsafe { std::mem::transmute((total & 0b00000011111111111111111111111111) as u32) };
        total >>= 26;
        let y: i16 = unsafe { std::mem::transmute((total & 0b0000001111111111) as u16) };
        total >>= 12;
        let x: i32 =
            unsafe { std::mem::transmute((total & 0b00000011111111111111111111111111) as u32) };

        Ok((Position { x, y, z }, input))
    }
}

impl<'a> MinecraftPacketPart<'a> for RawBytes<'a> {
    fn serialize_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
        output.extend_from_slice(self.data);
        Ok(())
    }

    fn deserialize_minecraft_packet_part(
        input: &'a mut [u8],
    ) -> Result<(Self, &'a mut [u8]), &'static str> {
        let data = input;
        Ok((RawBytes {data}, &mut []))
    }
}
