use super::*;
use std::collections::HashMap;

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

/// A length-prefixed modified UTF-8 string. The prefix is an unsigned short (thus 2 bytes) signifying the length of the string in bytes
#[inline]
pub fn parse_string(mut input: &mut [u8]) -> Result<(&str, &mut [u8]), &'static str> {
    if input.len() < 2 {
        return Err("A string tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    input = &mut input[2..];
    let (bytes, new_input) = input.split_at_mut(len);
    let string =
        std::str::from_utf8(bytes).map_err(|_| "A string should contain valid utf8 characters.")?;
    Ok((string, new_input))
}

/// A list of nameless tags, all of the same type. The list is prefixed with the Type ID of the items it contains (thus 1 byte), and the length of the list as a signed integer (a further 4 bytes).
#[inline]
pub fn parse_list(input: &mut [u8]) -> Result<(NbtList, &mut [u8]), &'static str> {
    if input.len() < 5 {
        return Err("A tag list should contain five bytes.");
    }
    let (tag_type, len): (u8, i32) = unsafe { (*input.get_unchecked(0), read_i32(&mut input[1..])) };
    if len <= 0 {
        return Ok((NbtList::None, &mut input[5..]));
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
            return Ok((NbtList::Byte(array), &mut input[5 + len..]));
        }
        2 => {
            if input.len() < 5 + len * 2 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i16, len) };
            return Ok((NbtList::Short(array), &mut input[5 + len * 2..]));
        }
        3 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i32, len) };
            return Ok((NbtList::Int(array), &mut input[5 + len * 4..]));
        }
        4 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const i64, len) };
            return Ok((NbtList::Long(array), &mut input[5 + len * 8..]));
        }
        5 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const f32, len) };
            return Ok((NbtList::Float(array), &mut input[5 + len * 4..]));
        }
        6 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let array =
                unsafe { std::slice::from_raw_parts(input.as_ptr().add(5) as *const f64, len) };
            return Ok((NbtList::Double(array), &mut input[5 + len * 8..]));
        }
        7 => {
            let mut input = &mut input[5..];
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
            let mut input = &mut input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) = parse_string(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::String(list), input))
        }
        9 => {
            let mut input = &mut input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) = parse_list(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::List(list), input))
        }
        10 => {
            let mut input = &mut input[5..];
            let mut list = Vec::new();
            for _ in 0..len {
                let (result, new_input) = parse_compound(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::Compound(list), input))
        }
        11 => {
            let mut input = &mut input[5..];
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
            let mut input = &mut input[5..];
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

/// A length-prefixed array of signed bytes. The prefix is a signed integer (thus 4 bytes)
#[inline]
pub fn parse_byte_array(input: &mut [u8]) -> Result<(&[i8], &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A byte array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len {
        return Err(
            "A byte array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i8, len) };
    Ok((array, &mut input[4 + len..]))
}

/// A length-prefixed array of signed integers. The prefix is a signed integer (thus 4 bytes) and indicates the number of 4 byte integers.
#[inline]
pub fn parse_int_array(input: &mut [u8]) -> Result<(&[i32], &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 4 {
        return Err("A int array tag cannot claim to contain more bytes than the remaining bytes.");
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i32, len) };
    Ok((array, &mut input[4 + len * 4..]))
}

/// A length-prefixed array of signed longs. The prefix is a signed integer (thus 4 bytes) and indicates the number of 8 byte longs.
#[inline]
pub fn parse_long_array(input: &mut [u8]) -> Result<(&[i64], &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A long array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((&[], &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 8 {
        return Err(
            "A long array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let array = unsafe { std::slice::from_raw_parts(input.as_ptr().add(4) as *const i64, len) };
    Ok((array, &mut input[4 + len * 8..]))
}
