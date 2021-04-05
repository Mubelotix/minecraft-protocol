#[inline]
pub(crate) unsafe fn read_i16(input: &mut [u8]) -> i16 {
    i16::from_be_bytes([*input.get_unchecked(0), *input.get_unchecked(1)])
}

#[inline]
pub(crate) unsafe fn read_u16(input: &mut [u8]) -> u16 {
    u16::from_be_bytes([*input.get_unchecked(0), *input.get_unchecked(1)])
}

#[inline]
pub(crate) unsafe fn read_i32(input: &mut [u8]) -> i32 {
    i32::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
    ])
}

#[inline]
pub(crate) unsafe fn read_i64(input: &mut [u8]) -> i64 {
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
pub(crate) unsafe fn read_f32(input: &mut [u8]) -> f32 {
    f32::from_be_bytes([
        *input.get_unchecked(0),
        *input.get_unchecked(1),
        *input.get_unchecked(2),
        *input.get_unchecked(3),
    ])
}

#[inline]
pub(crate) unsafe fn read_f64(input: &mut [u8]) -> f64 {
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
pub fn parse_byte(input: &mut [u8]) -> Result<(i8, &mut [u8]), &'static str> {
    let byte = *input.get(0).ok_or("A byte tag should contain a byte.")?;
    let byte = i8::from_be_bytes([byte]);
    Ok((byte, &mut input[1..]))
}

/// A single signed, big endian 16 bit integer
#[inline]
pub fn parse_short(input: &mut [u8]) -> Result<(i16, &mut [u8]), &'static str> {
    if input.len() < 2 {
        return Err("A short tag should contain two bytes.");
    }
    let value: i16 = unsafe { read_i16(input) };
    Ok((value, &mut input[2..]))
}

/// A single signed, big endian 32 bit integer
#[inline]
pub fn parse_int(input: &mut [u8]) -> Result<(i32, &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int tag should contain four bytes.");
    }
    let value: i32 = unsafe { read_i32(input) };
    Ok((value, &mut input[4..]))
}

/// A single signed, big endian 64 bit integer
#[inline]
pub fn parse_long(input: &mut [u8]) -> Result<(i64, &mut [u8]), &'static str> {
    if input.len() < 8 {
        return Err("A long tag should contain height bytes.");
    }
    let value: i64 = unsafe { read_i64(input) };
    Ok((value, &mut input[8..]))
}

/// A single, big endian IEEE-754 single-precision floating point number (NaN possible)
#[inline]
pub fn parse_float(input: &mut [u8]) -> Result<(f32, &mut [u8]), &'static str> {
    if input.len() < 4 {
        return Err("A float tag should contain four bytes.");
    }
    let value: f32 = unsafe { read_f32(input) };
    Ok((value, &mut input[4..]))
}

/// A single, big endian IEEE-754 double-precision floating point number (NaN possible)
#[inline]
pub fn parse_double(input: &mut [u8]) -> Result<(f64, &mut [u8]), &'static str> {
    if input.len() < 8 {
        return Err("A double tag should contain four bytes.");
    }
    let value: f64 = unsafe { read_f64(input) };
    Ok((value, &mut input[8..]))
}
