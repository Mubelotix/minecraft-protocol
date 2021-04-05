use std::{collections::HashMap, u8};

#[derive(Debug)]
pub enum NbtList<'a> {
    None,
    Byte(&'a [i8]),
    Short(&'a [i16]),
    Int(&'a [i32]),
    Long(&'a [i64]),
    Float(&'a [f32]),
    Double(&'a [f64]),
    ByteArray(Vec<&'a [i8]>),
    IntArray(Vec<&'a [i32]>),
    LongArray(Vec<&'a [i64]>),
    String(Vec<&'a str>),
    List(Vec<NbtList<'a>>),
    Compound(Vec<HashMap<&'a str, NbtTag<'a>>>),
}

#[derive(Debug)]
pub enum NbtTag<'a> {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(&'a [i8]),
    IntArray(&'a [i32]),
    LongArray(&'a [i64]),
    String(&'a str),
    List(NbtList<'a>),
    Compound(HashMap<&'a str, NbtTag<'a>>),
    RootCompound(&'a str, HashMap<&'a str, NbtTag<'a>>),
}

#[inline]
unsafe fn read_i16(input: &[u8]) -> i16 {
    i16::from_be_bytes([*input.get_unchecked(0), *input.get_unchecked(1)])
}

#[inline]
unsafe fn read_u16(input: &[u8]) -> u16 {
    u16::from_be_bytes([*input.get_unchecked(0), *input.get_unchecked(1)])
}

#[inline]
unsafe fn read_i32(input: &[u8]) -> i32 {
    i32::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
    ])
}

#[inline]
unsafe fn read_i64(input: &[u8]) -> i64 {
    i64::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
        *input.get_unchecked(4),
        *input.get_unchecked(5),
        *input.get_unchecked(6),
        *input.get_unchecked(7),
    ])
}

#[inline]
unsafe fn read_f32(input: &[u8]) -> f32 {
    f32::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
    ])
}

#[inline]
unsafe fn read_f64(input: &[u8]) -> f64 {
    f64::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
        *input.get_unchecked(4),
        *input.get_unchecked(5),
        *input.get_unchecked(6),
        *input.get_unchecked(7),
    ])
}

/// A single signed byte
#[inline]
pub fn parse_byte(input: &[u8]) -> Result<(i8, &[u8]), &'static str> {
    let byte = *input.get(0).ok_or("A byte tag should contain a byte.")?;
    let byte = i8::from_be_bytes([byte]);
    Ok((byte, &input[2..]))
}

/// A single signed, big endian 16 bit integer
#[inline]
pub fn parse_short(input: &[u8]) -> Result<(i16, &[u8]), &'static str> {
    if input.len() < 2 {
        return Err("A short tag should contain two bytes.");
    }
    let value: i16 = unsafe { read_i16(input) };
    Ok((value, &input[2..]))
}

/// A single signed, big endian 32 bit integer
#[inline]
pub fn parse_int(input: &[u8]) -> Result<(i32, &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int tag should contain four bytes.");
    }
    let value: i32 = unsafe { read_i32(input) };
    Ok((value, &input[4..]))
}

/// A single signed, big endian 64 bit integer
#[inline]
pub fn parse_long(input: &[u8]) -> Result<(i64, &[u8]), &'static str> {
    if input.len() < 8 {
        return Err("A long tag should contain height bytes.");
    }
    let value: i64 = unsafe { read_i64(input) };
    Ok((value, &input[8..]))
}

/// A single, big endian IEEE-754 single-precision floating point number (NaN possible)
#[inline]
pub fn parse_float(input: &[u8]) -> Result<(f32, &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A float tag should contain four bytes.");
    }
    let value: f32 = unsafe { read_f32(input) };
    Ok((value, &input[4..]))
}

/// A single, big endian IEEE-754 double-precision floating point number (NaN possible)
#[inline]
pub fn parse_double(input: &[u8]) -> Result<(f64, &[u8]), &'static str> {
    if input.len() < 8 {
        return Err("A double tag should contain four bytes.");
    }
    let value: f64 = unsafe { read_f64(input) };
    Ok((value, &input[8..]))
}

/// A length-prefixed array of signed bytes. The prefix is a signed integer (thus 4 bytes)
#[inline]
pub fn parse_byte_array(input: &[u8]) -> Result<(&[i8], &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A byte array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len {
        return Err(
            "A byte array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i8, len) };
    Ok((array, &input[4 + len..]))
}

/// A length-prefixed array of signed integers. The prefix is a signed integer (thus 4 bytes) and indicates the number of 4 byte integers.
#[inline]
pub fn parse_int_array(input: &[u8]) -> Result<(&[i32], &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 4 {
        return Err("A int array tag cannot claim to contain more bytes than the remaining bytes.");
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i32, len) };
    Ok((array, &input[4 + len * 4..]))
}

/// A length-prefixed array of signed longs. The prefix is a signed integer (thus 4 bytes) and indicates the number of 8 byte longs.
#[inline]
pub fn parse_long_array(input: &[u8]) -> Result<(&[i64], &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A long array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 8 {
        return Err(
            "A long array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i64, len) };
    Ok((array, &input[4 + len * 8..]))
}

/// A length-prefixed modified UTF-8 string. The prefix is an unsigned short (thus 2 bytes) signifying the length of the string in bytes
#[inline]
pub fn parse_string(input: &[u8]) -> Result<(&str, &[u8]), &'static str> {
    if input.len() < 2 {
        return Err("A string tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    let bytes = input
        .get(2..2 + len)
        .ok_or("A string tag cannot claim to contain more bytes than the remaining bytes.")?;
    let string =
        std::str::from_utf8(bytes).map_err(|_| "A string should contain valid utf8 characters.")?;
    Ok((string, &input[2 + len..]))
}

/// A list of nameless tags, all of the same type. The list is prefixed with the Type ID of the items it contains (thus 1 byte), and the length of the list as a signed integer (a further 4 bytes).
#[inline]
pub fn parse_list(input: &[u8]) -> Result<(NbtList, &[u8]), &'static str> {
    if input.len() < 5 {
        return Err("A tag list should contain five bytes.");
    }
    let (tag_type, len): (u8, i32) = unsafe { (*input.get_unchecked(0), read_i32(&input[1..])) };
    if len <= 0 {
        return Ok((NbtList::None, &input[5..]));
    }
    let len = len as usize;

    match tag_type {
        1 => {
            if input.len() < 5 + len {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i8, len) };
            return Ok((NbtList::Byte(array), &input[5 + len..]));
        }
        2 => {
            if input.len() < 5 + len * 2 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i16, len) };
            return Ok((NbtList::Short(array), &input[5 + len * 2..]));
        }
        3 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i32, len) };
            return Ok((NbtList::Int(array), &input[5 + len * 4..]));
        }
        4 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i64, len) };
            return Ok((NbtList::Long(array), &input[5 + len * 8..]));
        }
        5 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const f32, len) };
            return Ok((NbtList::Float(array), &input[5 + len * 4..]));
        }
        6 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const f64, len) };
            return Ok((NbtList::Double(array), &input[5 + len * 8..]));
        }
        7 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) =
                    parse_byte_array(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::ByteArray(list), input))
        }
        8 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) = parse_string(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::String(list), input))
        }
        9 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) = parse_list(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::List(list), input))
        }
        10 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) =
                    parse_coumpound(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::Compound(list), input))
        }
        11 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) =
                    parse_int_array(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::IntArray(list), input))
        }
        12 => {
            let mut input = &input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) =
                    parse_long_array(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::LongArray(list), input))
        }
        _ => Err("Unknown tag ID in list."),
    }
}

#[inline]
pub fn parse_coumpound(mut input: &[u8]) -> Result<(HashMap<&str, NbtTag>, &[u8]), &'static str> {
    let mut content = HashMap::new();

    loop {
        if input.first() == Some(&0) {
            input = &input[1..];
            break;
        }
        if input.len() < 3 {
            return Err("A tag in a compound should be introduced by three bytes.");
        }
        let (tag_id, len): (u8, u16) = unsafe { (*input.get_unchecked(0), read_u16(&input[1..])) };

        let len = len as usize;
        let bytes = input
            .get(3..3 + len)
            .ok_or("A tag name cannot claim to contain more bytes than the remaining bytes.")?;
        let name = std::str::from_utf8(bytes)
            .map_err(|_| "A tag name should contain valid utf8 characters.")?;
        let (tag, new_input) = parse_nbt_tag(&input[3 + len..], tag_id)?;
        input = new_input;
        content.insert(name, tag);
    }

    Ok((content, input))
}

pub fn parse_root_coumpound(
    mut input: &[u8],
) -> Result<((&str, HashMap<&str, NbtTag>), &[u8]), &'static str> {
    if input.first() != Some(&10) {
        return Err("The root compound tag should start with the coumpound ID (10).");
    }
    input = &input[1..];
    if input.len() < 2 {
        return Err("A root compound tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    let bytes = input.get(2..2 + len).ok_or(
        "A root coumpound tag name cannot claim to contain more bytes than the remaining bytes.",
    )?;
    let name = std::str::from_utf8(bytes)
        .map_err(|_| "A coumpound tag name should contain valid utf8 characters.")?;
    input = &input[2 + len..];

    let (content, input) = parse_coumpound(input)?;

    Ok(((name, content), input))
}

#[inline]
pub fn parse_nbt_tag(input: &[u8], tag_id: u8) -> Result<(NbtTag, &[u8]), &'static str> {
    match tag_id {
        1 => {
            let (value, remaining_bytes) = parse_byte(input)?;
            Ok((NbtTag::Byte(value), remaining_bytes))
        }
        2 => {
            let (value, remaining_bytes) = parse_short(input)?;
            Ok((NbtTag::Short(value), remaining_bytes))
        }
        3 => {
            let (value, remaining_bytes) = parse_int(input)?;
            Ok((NbtTag::Int(value), remaining_bytes))
        }
        4 => {
            let (value, remaining_bytes) = parse_long(input)?;
            Ok((NbtTag::Long(value), remaining_bytes))
        }
        5 => {
            let (value, remaining_bytes) = parse_float(input)?;
            Ok((NbtTag::Float(value), remaining_bytes))
        }
        6 => {
            let (value, remaining_bytes) = parse_double(input)?;
            Ok((NbtTag::Double(value), remaining_bytes))
        }
        7 => {
            let (value, remaining_bytes) = parse_byte_array(input)?;
            Ok((NbtTag::ByteArray(value), remaining_bytes))
        }
        11 => {
            let (value, remaining_bytes) = parse_int_array(input)?;
            Ok((NbtTag::IntArray(value), remaining_bytes))
        }
        12 => {
            let (value, remaining_bytes) = parse_long_array(input)?;
            Ok((NbtTag::LongArray(value), remaining_bytes))
        }
        8 => {
            let (value, remaining_bytes) = parse_string(input)?;
            Ok((NbtTag::String(value), remaining_bytes))
        }
        9 => {
            let (value, remaining_bytes) = parse_list(input)?;
            Ok((NbtTag::List(value), remaining_bytes))
        }
        10 => {
            let (value, remaining_bytes) = parse_coumpound(input)?;
            Ok((NbtTag::Compound(value), remaining_bytes))
        }
        _ => Err("Unknown tag ID"),
    }
}

pub fn parse_nbt(input: &[u8]) -> Result<(NbtTag, &[u8]), &'static str> {
    let tag_id = *input.first().ok_or("Empty input, no NBT data.")?;
    parse_nbt_tag(&input[1..], tag_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbt() {
        println!("{:?}", parse_root_coumpound(include_bytes!("test_data/bigtest.nbt")).unwrap());
        println!("{:?}", parse_root_coumpound(include_bytes!("test_data/hello_world.nbt")).unwrap());
    }
}
