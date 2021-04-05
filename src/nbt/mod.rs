use std::collections::HashMap;
pub mod arrays;
pub mod compound;
pub mod numbers;
use arrays::*;
use compound::*;
use numbers::*;

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
            let (value, remaining_bytes) = parse_compound(input)?;
            Ok((NbtTag::Compound(value), remaining_bytes))
        }
        _ => Err("Unknown tag ID"),
    }
}

pub fn parse_nbt(input: &[u8]) -> Result<(NbtTag, &[u8]), &'static str> {
    let tag_id = *input.first().ok_or("Empty input, no NBT data.")?;
    if tag_id == 10 {
        let (name, content) = parse_root_compound(input)?;
        return Ok((NbtTag::RootCompound(name, content), &[]));
    }
    parse_nbt_tag(&input[1..], tag_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbt() {
        println!(
            "{:?}",
            parse_nbt(include_bytes!("test_data/bigtest.nbt")).unwrap()
        );
        println!(
            "{:?}",
            parse_nbt(include_bytes!("test_data/hello_world.nbt")).unwrap()
        );
    }
}
