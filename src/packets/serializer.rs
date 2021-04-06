use std::convert::TryInto;

use super::*;

pub trait MinecraftPacketPart: Sized {
    fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str>;
    fn build_from_minecraft_packet(input: &mut [u8]) -> Result<(Self, &mut [u8]), &'static str>;
}

pub trait MinecraftPacket: Sized {
    fn serialize(self) -> Result<Vec<u8>, &'static str>;
    fn deserialize(input: &mut [u8]) -> Result<Self, &'static str>;
}

mod integers {
    use super::*;

    impl MinecraftPacketPart for bool {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self as u8);
            Ok(())
        }

        fn build_from_minecraft_packet(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing bool.")?;
            Ok((*value != 0, input))
        }
    }

    impl MinecraftPacketPart for i8 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self.to_le_bytes()[0]);
            Ok(())
        }

        fn build_from_minecraft_packet(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing i8.")?;
            Ok((i8::from_le_bytes([*value]), input))
        }
    }

    impl MinecraftPacketPart for u8 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            output.push(self);
            Ok(())
        }

        fn build_from_minecraft_packet(
            input: &mut [u8],
        ) -> Result<(Self, &mut [u8]), &'static str> {
            let (value, input) = input
                .split_first_mut()
                .ok_or("Missing byte while parsing u8.")?;
            Ok((*value, input))
        }
    }

    impl MinecraftPacketPart for i16 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for u16 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for i32 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for i64 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
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

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for f32 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
            let bytes = self.to_le_bytes();
            output.push(bytes[3]);
            output.push(bytes[2]);
            output.push(bytes[1]);
            output.push(bytes[0]);
            Ok(())
        }

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for f64 {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
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

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for VarInt {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
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

        fn build_from_minecraft_packet(
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

    impl MinecraftPacketPart for VarLong {
        fn append_minecraft_packet_part(self, output: &mut Vec<u8>) -> Result<(), &'static str> {
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

        fn build_from_minecraft_packet(
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
                    .append_minecraft_packet_part(&mut buffer)
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
                let (result, _) = VarInt::build_from_minecraft_packet(inputs[idx]).unwrap();
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
                    .append_minecraft_packet_part(&mut buffer)
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
                let (result, _) = VarLong::build_from_minecraft_packet(inputs[idx]).unwrap();
                outputs.push(result.0);
            }

            for idx in 0..inputs.len() {
                assert_eq!(outputs[idx], expected_outputs[idx]);
            }
        }
    }
}

