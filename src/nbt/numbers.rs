/// A single signed byte
#[inline]
pub fn parse_byte(input: &[u8]) -> Result<(i8, &[u8]), &'static str> {
    let byte = *input.get(0).ok_or("A byte tag should contain a byte.")?;
    let byte = i8::from_be_bytes([byte]);
    Ok((byte, &input[1..]))
}

/// A single signed, big endian 16 bit integer
#[inline]
pub fn parse_short(input: &[u8]) -> Result<(i16, &[u8]), &'static str> {
    if input.len() < 2 {
        return Err("A short tag should contain two bytes.");
    }
    Ok(unsafe {
        (
            i16::from_be_bytes(*(input.as_ptr() as *mut [u8; 2])),
            input.get_unchecked(2..),
        )
    })
}

/// A single signed, big endian 32 bit integer
#[inline]
pub fn parse_int(input: &[u8]) -> Result<(i32, &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A int tag should contain four bytes.");
    }
    Ok(unsafe {
        (
            i32::from_be_bytes(*(input.as_ptr() as *mut [u8; 4])),
            input.get_unchecked(4..),
        )
    })
}

/// A single signed, big endian 64 bit integer
#[inline]
pub fn parse_long(input: &[u8]) -> Result<(i64, &[u8]), &'static str> {
    if input.len() < 8 {
        return Err("A long tag should contain height bytes.");
    }
    Ok(unsafe {
        (
            i64::from_be_bytes(*(input.as_ptr() as *mut [u8; 8])),
            input.get_unchecked(8..),
        )
    })
}

/// A single, big endian IEEE-754 single-precision floating point number (NaN possible)
#[inline]
pub fn parse_float(input: &[u8]) -> Result<(f32, &[u8]), &'static str> {
    if input.len() < 4 {
        return Err("A float tag should contain four bytes.");
    }
    unsafe {
        let number = input.get_unchecked(..4);
        #[cfg(target_endian = "little")]
        let number = f32::from_be_bytes([
            *number.get_unchecked(0),
            *number.get_unchecked(1),
            *number.get_unchecked(2),
            *number.get_unchecked(3),
        ]);

        Ok((number, input.get_unchecked(4..)))
    }
}

/// A single, big endian IEEE-754 double-precision floating point number (NaN possible)
#[inline]
pub fn parse_double(input: &[u8]) -> Result<(f64, &[u8]), &'static str> {
    if input.len() < 8 {
        return Err("A double tag should contain four bytes.");
    }
    unsafe {
        let number = input.get_unchecked(..8);
        #[cfg(target_endian = "little")]
        let number = f64::from_be_bytes([
            *number.get_unchecked(0),
            *number.get_unchecked(1),
            *number.get_unchecked(2),
            *number.get_unchecked(3),
            *number.get_unchecked(4),
            *number.get_unchecked(5),
            *number.get_unchecked(6),
            *number.get_unchecked(7),
        ]);

        Ok((number, input.get_unchecked(8..)))
    }
}
