use super::*;
use std::collections::HashMap;

#[inline]
pub fn parse_compound(mut input: &[u8]) -> Result<(HashMap<String, NbtTag>, &[u8]), &'static str> {
    let mut content = HashMap::new();

    loop {
        if input.first() == Some(&0) {
            input = &input[1..];
            break;
        }
        if input.len() < 3 {
            return Err("A tag in a compound should be introduced by three bytes.");
        }
        let (tag_id, len): (u8, u16) = unsafe {
            (
                *input.get_unchecked(0),
                u16::from_be(*(input.as_ptr().add(1) as *mut u16)),
            )
        };

        let len = len as usize;
        let new_input = &input[3..];
        let (bytes, new_input) = new_input.split_at(len);
        let name = String::from_utf8(bytes.to_vec())
            .map_err(|_| "A tag name should contain valid utf8 characters.")?;
        let (tag, new_input) = parse_nbt_tag(new_input, tag_id)?;
        input = new_input;
        content.insert(name, tag);
    }

    Ok((content, input))
}

#[allow(clippy::type_complexity)]
pub fn parse_root_compound(
    mut input: &[u8],
) -> Result<((String, HashMap<String, NbtTag>), &[u8]), &'static str> {
    if input.first() != Some(&10) {
        return Err("The root compound tag should start with the compound ID (10).");
    }
    input = &input[1..];
    if input.len() < 2 {
        return Err("A root compound tag should contain two bytes.");
    }
    let len: u16 = unsafe { u16::from_be(*(input.as_ptr() as *mut u16)) };
    let len = len as usize;
    input = &input[2..];
    let (bytes, new_input) = input.split_at(len);
    let name = String::from_utf8(bytes.to_vec())
        .map_err(|_| "A compound tag name should contain valid utf8 characters.")?;
    input = new_input;

    let (content, input) = parse_compound(input)?;

    Ok(((name, content), input))
}

pub fn parse_root_compound_complete(
    input: &[u8],
) -> Result<(String, HashMap<String, NbtTag>), &'static str> {
    let (value, input) = parse_root_compound(input)?;

    if !input.is_empty() {
        return Err("There should be no data after a root compound.");
    }

    Ok(value)
}
