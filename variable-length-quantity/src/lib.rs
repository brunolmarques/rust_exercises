#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();

    for value in values {
        // get the first seven bits
        let mut bytes = vec![(value & 0x7f) as u8];
        // process remaining bits, if any
        let mut number: u32 = value >> 7;

        while number != 0 {
            // get the first seven bits and set the series bit
            bytes.insert(0, (number & 0x7f | 0x80) as u8);
            number >>= 7;
        }
        // add the bytes to the result
        result.extend(bytes);
    }

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut number: u32 = 0;

    for (idx, byte) in bytes.iter().enumerate() {
        // test if prior round has overflowed
        if number & 0xfe_00_00_00 > 0 {
            return Err(Error::Overflow);
        }

        // add the significant bits to the running total
        number <<= 7;
        number |= u32::from(byte & 0x7f);
        // if the series bit is unset, we've reached the end of the number

        if 0x80 & byte == 0 {
            result.push(number);
            number = 0;
            // or if not we may be dealing with malformed data
        } else if idx + 1 == bytes.len() {
            return Err(Error::IncompleteNumber);
        }
    }

    Ok(result)
}