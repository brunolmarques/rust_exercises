use crate::Error::InvalidDigit;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
use Error::*;

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        Err(InvalidInputBase)
    } else if to_base < 2 {
        Err(InvalidOutputBase)
    } else {
        let num = to_decimal(number, from_base)?;
        Ok(from_decimal(num, to_base))
    }
}

fn to_decimal(digits: &[u32], from_base: u32) -> Result<u32, Error> {
    digits
        .iter()
        .fold(Ok(0), |acc, &digit| {
            if digit >= from_base {
                return Err(InvalidDigit(digit));
            }
            acc.map(|num| from_base * num + digit)
        })
}

fn from_decimal(num: u32, to_base: u32) -> Vec<u32> {
    let mut digits = Vec::with_capacity(1);
    let mut num = num;

    while num > 0 {
        digits.insert(0, num % to_base);
        num /= to_base;
    }
    if digits.is_empty() {
        digits.push(0)
    }
    digits
}




