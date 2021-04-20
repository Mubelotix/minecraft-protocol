use std::collections::HashMap;
pub mod arrays;
pub mod compound;
pub mod numbers;
pub mod serializer;
use arrays::*;
use compound::*;
use numbers::*;

#[derive(Debug)]
pub enum NbtTag {
    Null,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
    String(String),
    List(NbtList),
    Compound(HashMap<String, NbtTag>),
    RootCompound(String, HashMap<String, NbtTag>),
}

#[inline]
pub fn parse_nbt_tag(input: &[u8], tag_id: u8) -> Result<(NbtTag, &[u8]), &'static str> {
    match tag_id {
        0 => Ok((NbtTag::Null, input)),
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

impl NbtTag {
    pub fn serialize_type_id(&self, output: &mut Vec<u8>) {
        output.push(match self {
            NbtTag::Null => 0,
            NbtTag::Byte(_) => 1,
            NbtTag::Short(_) => 2,
            NbtTag::Int(_) => 3,
            NbtTag::Long(_) => 4,
            NbtTag::Float(_) => 5,
            NbtTag::Double(_) => 6,
            NbtTag::ByteArray(_) => 7,
            NbtTag::IntArray(_) => 11,
            NbtTag::LongArray(_) => 12,
            NbtTag::String(_) => 8,
            NbtTag::List(_) => 9,
            NbtTag::Compound(_) => 10,
            NbtTag::RootCompound(_, _) => 10,
        });
    }
    
    pub fn serialize_value(&self, output: &mut Vec<u8>) {
        match self {
            NbtTag::Null => (),
            NbtTag::Byte(byte) => {
                output.push(byte.to_be_bytes()[0]);
            },
            NbtTag::Short(short) => {
                output.extend_from_slice(&short.to_be_bytes());
            }
            NbtTag::Int(int) => {
                output.extend_from_slice(&int.to_be_bytes());
            }
            NbtTag::Long(long) => {
                output.extend_from_slice(&long.to_be_bytes());
            }
            NbtTag::Float(float) => {
                output.extend_from_slice(&float.to_be_bytes());
            }
            NbtTag::Double(double) => {
                output.extend_from_slice(&double.to_be_bytes());
            }
            NbtTag::ByteArray(array) => {
                output.extend_from_slice(&(array.len() as i32).to_be_bytes());
                for byte in array {
                    output.push(byte.to_be_bytes()[0]);
                }
            }
            NbtTag::IntArray(array) => {
                output.extend_from_slice(&(array.len() as i32).to_be_bytes());
                for int in array {
                    output.extend_from_slice(&int.to_be_bytes());
                }
            }
            NbtTag::LongArray(array) => {
                output.extend_from_slice(&(array.len() as i32).to_be_bytes());
                for long in array {
                    output.extend_from_slice(&long.to_be_bytes());
                }
            }
            NbtTag::String(string) => {
                output.extend_from_slice(&(string.len() as u16).to_be_bytes());
                output.extend_from_slice(string.as_bytes());
            }
            NbtTag::List(list) => {
                list.serialize_list(output);
            }
            NbtTag::Compound(compound) => {
                for (name, value) in compound.iter() {
                    value.serialize_type_id(output);
                    output.extend_from_slice(&(name.len() as u16).to_be_bytes());
                    output.extend_from_slice(name.as_bytes());
                    value.serialize_value( output);
                }
                output.push(0);
            }
            NbtTag::RootCompound(name, compound) => {
                output.extend_from_slice(&(name.len() as u16).to_be_bytes());
                output.extend_from_slice(name.as_bytes());
                for (name, value) in compound.iter() {
                    value.serialize_type_id(output);
                    output.extend_from_slice(&(name.len() as u16).to_be_bytes());
                    output.extend_from_slice(name.as_bytes());
                    value.serialize_value( output);
                }
                output.push(0);
            }
        }
    }

    pub fn serialize(&self, output: &mut Vec<u8>) {
        self.serialize_type_id(output);
        self.serialize_value(output);
    }
}

pub fn parse_nbt(input: &[u8]) -> Result<(NbtTag, &[u8]), &'static str> {
    let tag_id = *input.first().ok_or("Empty input, no NBT data.")?;
    if tag_id == 10 {
        let ((name, content), input) = parse_root_compound(input)?;
        return Ok((NbtTag::RootCompound(name, content), input));
    }
    parse_nbt_tag(&input[1..], tag_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nbt() {
        println!(
            "{:#?}",
            parse_nbt(&include_bytes!("test_data/bigtest.nbt").to_vec()).unwrap()
        );
        println!(
            "{:#?}",
            parse_nbt(&include_bytes!("test_data/hello_world.nbt").to_vec()).unwrap()
        );
        println!(
            "{:#?}",
            parse_nbt(&include_bytes!("test_data/servers.dat").to_vec()).unwrap()
        );
        println!(
            "{:#?}",
            parse_nbt(&include_bytes!("test_data/level.dat").to_vec()).unwrap()
        );
    }
}
