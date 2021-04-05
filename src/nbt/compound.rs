use super::*;
use std::collections::HashMap;

#[inline]
pub fn parse_compound(mut input: &[u8]) -> Result<(HashMap<&str, NbtTag>, &[u8]), &'static str> {
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

pub fn parse_root_compound(
    mut input: &[u8],
) -> Result<(&str, HashMap<&str, NbtTag>), &'static str> {
    if input.first() != Some(&10) {
        return Err("The root compound tag should start with the compound ID (10).");
    }
    input = &input[1..];
    if input.len() < 2 {
        return Err("A root compound tag should contain two bytes.");
    }
    let len: u16 = unsafe { read_u16(input) };
    let len = len as usize;
    let bytes = input.get(2..2 + len).ok_or(
        "A root compound tag name cannot claim to contain more bytes than the remaining bytes.",
    )?;
    let name = std::str::from_utf8(bytes)
        .map_err(|_| "A compound tag name should contain valid utf8 characters.")?;
    input = &input[2 + len..];


    let (content, input) = parse_compound(input)?;
    if !input.is_empty() {
        return Err("There should be no data after a root compound.");
    }

    Ok((name, content))
}
