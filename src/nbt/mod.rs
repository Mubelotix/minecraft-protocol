use std::collections::HashMap;
pub mod arrays;
pub mod compound;
pub mod numbers;
pub mod serializer;
use arrays::*;
use compound::*;
use numbers::*;

#[derive(Debug, Clone, PartialEq)]
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

impl NbtTag {
    #[inline]
    pub fn as_null(&mut self) -> Option<()> {
        if let NbtTag::Null = self {
            Some(())
        } else {
            None
        }
    }

    #[inline]
    pub fn as_byte(&self) -> Option<&i8> {
        if let NbtTag::Byte(byte) = self {
            Some(byte)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_byte(&mut self) -> Option<&mut i8> {
        if let NbtTag::Byte(byte) = self {
            Some(byte)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_short(&self) -> Option<&i16> {
        if let NbtTag::Short(short) = self {
            Some(short)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_short(&mut self) -> Option<&mut i16> {
        if let NbtTag::Short(short) = self {
            Some(short)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_int(&self) -> Option<&i32> {
        if let NbtTag::Int(int) = self {
            Some(int)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_int(&mut self) -> Option<&mut i32> {
        if let NbtTag::Int(int) = self {
            Some(int)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_long(&self) -> Option<&i64> {
        if let NbtTag::Long(long) = self {
            Some(long)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_long(&mut self) -> Option<&mut i64> {
        if let NbtTag::Long(long) = self {
            Some(long)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_float(&self) -> Option<&f32> {
        if let NbtTag::Float(float) = self {
            Some(float)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_float(&mut self) -> Option<&mut f32> {
        if let NbtTag::Float(float) = self {
            Some(float)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_double(&self) -> Option<&f64> {
        if let NbtTag::Double(double) = self {
            Some(double)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_double(&mut self) -> Option<&mut f64> {
        if let NbtTag::Double(double) = self {
            Some(double)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_byte_array(&self) -> Option<&Vec<i8>> {
        if let NbtTag::ByteArray(byte_array) = self {
            Some(byte_array)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_byte_array(&mut self) -> Option<&mut Vec<i8>> {
        if let NbtTag::ByteArray(byte_array) = self {
            Some(byte_array)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_int_array(&self) -> Option<&Vec<i32>> {
        if let NbtTag::IntArray(int_array) = self {
            Some(int_array)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_int_array(&mut self) -> Option<&mut Vec<i32>> {
        if let NbtTag::IntArray(int_array) = self {
            Some(int_array)
        } else {
            None
        }
    }
    
    #[inline]
    pub fn as_long_array(&self) -> Option<&Vec<i64>> {
        if let NbtTag::LongArray(long_array) = self {
            Some(long_array)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_long_array(&mut self) -> Option<&mut Vec<i64>> {
        if let NbtTag::LongArray(long_array) = self {
            Some(long_array)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_string(&self) -> Option<&String> {
        if let NbtTag::String(string) = self {
            Some(string)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_string(&mut self) -> Option<&mut String> {
        if let NbtTag::String(string) = self {
            Some(string)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_list(&self) -> Option<&NbtList> {
        if let NbtTag::List(list) = self {
            Some(list)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_list(&mut self) -> Option<&mut NbtList> {
        if let NbtTag::List(list) = self {
            Some(list)
        } else {
            None
        }
    }

    #[inline]
    pub fn as_compound(&self) -> Option<&HashMap<String, NbtTag>> {
        if let NbtTag::Compound(compound) = self {
            Some(compound)
        } else if let NbtTag::RootCompound(_name, compound) = self {
            Some(compound)
        } else {
            None
        }
    }
    #[inline]
    pub fn as_mut_compound(&mut self) -> Option<&mut HashMap<String, NbtTag>> {
        if let NbtTag::Compound(compound) = self {
            Some(compound)
        } else if let NbtTag::RootCompound(_name, compound) = self {
            Some(compound)
        } else {
            None
        }
    }
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
        let original = include_bytes!("test_data/bigtest.nbt").to_vec();
        let parsed = parse_nbt(&original).unwrap().0;
        let mut serialized = Vec::new();
        parsed.serialize(&mut serialized);
        let parsed2 = parse_nbt(&serialized).unwrap().0;
        assert_eq!(parsed, parsed2);

        let original = include_bytes!("test_data/hello_world.nbt").to_vec();
        let parsed = parse_nbt(&original).unwrap().0;
        let mut serialized = Vec::new();
        parsed.serialize(&mut serialized);
        let parsed2 = parse_nbt(&serialized).unwrap().0;
        assert_eq!(parsed, parsed2);

        let original = include_bytes!("test_data/servers.dat").to_vec();
        let parsed = parse_nbt(&original).unwrap().0;
        let mut serialized = Vec::new();
        parsed.serialize(&mut serialized);
        let parsed2 = parse_nbt(&serialized).unwrap().0;
        assert_eq!(parsed, parsed2);

        let original = include_bytes!("test_data/level.dat").to_vec();
        let parsed = parse_nbt(&original).unwrap().0;
        let mut serialized = Vec::new();
        parsed.serialize(&mut serialized);
        let parsed2 = parse_nbt(&serialized).unwrap().0;
        assert_eq!(parsed, parsed2)
    }
}
