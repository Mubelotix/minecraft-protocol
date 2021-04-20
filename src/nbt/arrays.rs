use super::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum NbtList {
    None,
    Byte(Vec<i8>),
    Short(Vec<i16>),
    Int(Vec<i32>),
    Long(Vec<i64>),
    Float(Vec<f32>),
    Double(Vec<f64>),
    ByteArray(Vec<Vec<i8>>),
    IntArray(Vec<Vec<i32>>),
    LongArray(Vec<Vec<i64>>),
    String(Vec<String>),
    List(Vec<NbtList>),
    Compound(Vec<HashMap<String, NbtTag>>),
}

/// A length-prefixed modified UTF-8 string. The prefix is an unsigned short (thus 2 bytes) signifying the length of the string in bytes
#[inline]
pub fn parse_string(mut input: &mut [u8]) -> Result<(String, &mut [u8]), &'static str> {
    if input.len() < 2 {
        return Err("A string tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    input = &mut input[2..];
    let (bytes, new_input) = input.split_at_mut(len);
    let string = String::from_utf8(bytes.to_vec())
        .map_err(|_| "A string should contain valid utf8 characters.")?;
    Ok((string, new_input))
}

/// A list of nameless tags, all of the same type. The list is prefixed with the Type ID of the items it contains (thus 1 byte), and the length of the list as a signed integer (a further 4 bytes).
#[inline]
pub fn parse_list(input: &mut [u8]) -> Result<(NbtList, &mut [u8]), &'static str> {
    if input.len() < 5 {
        return Err("A tag list should contain five bytes.");
    }
    let (tag_type, len): (u8, i32) =
        unsafe { (*input.get_unchecked(0), read_i32(&mut input[1..])) };
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
            Ok((NbtList::Byte(array.to_vec()), &mut input[5 + len..]))
        }
        2 => {
            if input.len() < 5 + len * 2 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let mut array = Vec::with_capacity(len);
            for i in 0..len {
                let element = unsafe { i16::from_be(*(input.as_ptr().add(5 + 2 * i) as *mut i16)) };
                array.push(element);
            }
            Ok((NbtList::Short(array), &mut input[5 + len * 2..]))
        }
        3 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let mut array = Vec::with_capacity(len);
            for i in 0..len {
                let element = unsafe { i32::from_be(*(input.as_ptr().add(5 + 4 * i) as *mut i32)) };
                array.push(element);
            }
            Ok((NbtList::Int(array), &mut input[5 + len * 4..]))
        }
        4 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let mut array = Vec::with_capacity(len);
            for i in 0..len {
                let element = unsafe { i64::from_be(*(input.as_ptr().add(5 + 8 * i) as *mut i64)) };
                array.push(element);
            }
            Ok((NbtList::Long(array), &mut input[5 + len * 8..]))
        }
        5 => {
            if input.len() < 5 + len * 4 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let mut array = Vec::with_capacity(len);
            for i in 0..len {
                unsafe {
                    let element = input.get_unchecked(5 + 4 * i..5 + 4 * i + 4);
                    #[cfg(target_endian = "little")]
                    let element = f32::from_be_bytes([
                        *element.get_unchecked(0),
                        *element.get_unchecked(1),
                        *element.get_unchecked(2),
                        *element.get_unchecked(3),
                    ]);

                    array.push(element);
                }
            }
            Ok((NbtList::Float(array), &mut input[5 + len * 4..]))
        }
        6 => {
            if input.len() < 5 + len * 8 {
                return Err(
                    "A list tag cannot claim to contain more bytes than the remaining bytes.",
                );
            }
            let mut array = Vec::with_capacity(len);
            for i in 0..len {
                unsafe {
                    let element = input.get_unchecked(5 + 8 * i..5 + 8 * i + 8);
                    #[cfg(target_endian = "little")]
                    let element = f64::from_be_bytes([
                        *element.get_unchecked(0),
                        *element.get_unchecked(1),
                        *element.get_unchecked(2),
                        *element.get_unchecked(3),
                        *element.get_unchecked(4),
                        *element.get_unchecked(5),
                        *element.get_unchecked(6),
                        *element.get_unchecked(7),
                    ]);
                    array.push(element);
                }
            }
            Ok((NbtList::Double(array), &mut input[5 + len * 8..]))
        }
        7 => {
            let mut input = &mut input[5..];
            let mut list = Vec::with_capacity(len);
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
            let mut list = Vec::with_capacity(len);
            for _ in 0..len {
                let (result, new_input) = parse_string(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::String(list), input))
        }
        9 => {
            let mut input = &mut input[5..];
            let mut list = Vec::with_capacity(len);
            for _ in 0..len {
                let (result, new_input) = parse_list(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::List(list), input))
        }
        10 => {
            let mut input = &mut input[5..];
            let mut list = Vec::with_capacity(len);
            for _ in 0..len {
                let (result, new_input) = parse_compound(input).map_err(|_| "Invalid list item")?;
                input = new_input;
                list.push(result);
            }
            Ok((NbtList::Compound(list), input))
        }
        11 => {
            let mut input = &mut input[5..];
            let mut list = Vec::with_capacity(len);
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
            let mut list = Vec::with_capacity(len);
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
pub fn parse_byte_array(input: &mut [u8]) -> Result<(Vec<i8>, &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A byte array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((Vec::new(), &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len {
        return Err(
            "A byte array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let mut array = Vec::with_capacity(len);
    for i in 0..len {
        unsafe {
            let element = *(input.as_ptr().add(4 + i) as *mut i8);
            array.push(element);
        }
    }
    Ok((array, &mut input[4 + len..]))
}

/// A length-prefixed array of signed integers. The prefix is a signed integer (thus 4 bytes) and indicates the number of 4 byte integers.
#[inline]
pub fn parse_int_array(input: &mut [u8]) -> Result<(Vec<i32>, &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((Vec::new(), &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 4 {
        return Err("A int array tag cannot claim to contain more bytes than the remaining bytes.");
    }
    let mut array = Vec::with_capacity(len);
    for i in 0..len {
        unsafe {
            let element = i32::from_be(*(input.as_ptr().add(4 + 4 * i) as *mut i32));
            array.push(element);
        }
    }
    Ok((array, &mut input[4 + len * 4..]))
}

/// A length-prefixed array of signed longs. The prefix is a signed integer (thus 4 bytes) and indicates the number of 8 byte longs.
#[inline]
pub fn parse_long_array(input: &mut [u8]) -> Result<(Vec<i64>, &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A long array tag should contain four bytes.");
    }
    let len: i32 = unsafe { read_i32(input) };
    if len <= 0 {
        return Ok((Vec::new(), &mut input[4..]));
    }
    let len = len as usize;
    if input.len() < 4 + len * 8 {
        return Err(
            "A long array tag cannot claim to contain more bytes than the remaining bytes.",
        );
    }
    let mut array = Vec::with_capacity(len);
    for i in 0..len {
        unsafe {
            let element = i64::from_be(*(input.as_ptr().add(4 + 8 * i) as *mut i64));
            array.push(element);
        }
    }
    Ok((array, &mut input[4 + len * 8..]))
}
