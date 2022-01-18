use std::cmp::Ordering;
use std::iter;
use std::ops::{Add, Mul, Sub};
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Sign {
    Positive,
    Negative,
}
impl Default for Sign {
    fn default() -> Self {
        Sign::Positive
    }
}
impl Sign {
    fn reverse(self) -> Sign {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}
/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Decimal {
    numbers: Vec<u8>,
    decimal_size: usize, // Position is from right
    sign: Sign,
}
fn string_to_vec(input: &str) -> Vec<u8> {
    input
        .chars()
        .fold(Vec::with_capacity(input.len()), |mut v, c| {
            if c.is_numeric() {
                v.push(c.to_digit(10).unwrap() as u8);
            }
            v
        })
}
impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        let dot_position = input.rfind('.').or_else(|| Some(input.len() - 1)).unwrap() + 1;
        let decimal_size = input.len() - dot_position;
        let sign = if input.starts_with('-') {
            Sign::Negative
        } else {
            Sign::Positive
        };
        println!("{:?} {:?} {:?}", dot_position, decimal_size, sign);
        let numbers = string_to_vec(input);
        let normalized = normalize(Self {
            numbers,
            decimal_size,
            sign,
        });
        Some(normalized)
    }
    pub fn inverse(self) -> Decimal {
        Decimal {
            sign: self.sign.reverse(),
            ..self
        }
    }
}
impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let (a, b, _) = equalize(&self, &other);
        println!("{:?} {:?}", a, b);
        match (self.sign, other.sign) {
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Positive, Sign::Positive) => a.numbers.cmp(&b.numbers),
            (Sign::Negative, Sign::Negative) => a.numbers.cmp(&b.numbers).reverse(),
        }
    }
}
const PAD: u8 = 0;
fn normalize(number: Decimal) -> Decimal {
    let numbers = number.numbers;
    let decimal_size = number.decimal_size;
    let int_size = numbers.len() - decimal_size;
    let int_part: Vec<&u8> = numbers
        .iter()
        .take(int_size)
        .skip_while(|&&i| i == 0)
        .collect();
    let dec_part: Vec<&u8> = numbers
        .iter()
        .rev()
        .take(decimal_size)
        .skip_while(|&&i| i == 0)
        .collect();
    let decimal_size = dec_part.len();
    let full_numbers: Vec<u8> = int_part
        .iter()
        .chain(dec_part.iter().rev())
        .map(|i| **i)
        .collect();
    Decimal {
        numbers: full_numbers,
        decimal_size,
        sign: number.sign,
    }
}
fn pad(input: &[u8], before: usize, after: usize) -> Vec<u8> {
    let result = iter::repeat(&PAD)
        .take(after)
        .chain(input.iter().rev())
        .chain(iter::repeat(&PAD).take(before + 1));
    let mut result: Vec<_> = result.copied().collect();
    result.reverse();
    result
}
fn equalize(number_1: &Decimal, number_2: &Decimal) -> (Decimal, Decimal, usize) {
    let longest_after_dot = number_1.decimal_size.max(number_2.decimal_size);
    let add_after_n1 = longest_after_dot - number_1.decimal_size;
    let add_after_n2 = longest_after_dot - number_2.decimal_size;
    let adjusted_n1_len = number_1.numbers.len() + add_after_n1;
    let adjusted_n2_len = number_2.numbers.len() + add_after_n2;
    let longest_total = adjusted_n1_len.max(adjusted_n2_len);
    let add_before_n1 = longest_total - adjusted_n1_len;
    let add_before_n2 = longest_total - adjusted_n2_len;
    let first_number = pad(&number_1.numbers, add_before_n1, add_after_n1);
    let second_number = pad(&number_2.numbers, add_before_n2, add_after_n2);
    (
        Decimal {
            numbers: first_number,
            decimal_size: longest_after_dot,
            sign: number_1.sign,
        },
        Decimal {
            numbers: second_number,
            decimal_size: longest_after_dot,
            sign: number_2.sign,
        },
        longest_after_dot,
    )
}
type ComputeFunction = fn(u8, u8, u8) -> (u8, u8);
fn accumulate(
    first_decimal: &Decimal,
    second_decimal: &Decimal,
    compute: ComputeFunction,
) -> (Vec<u8>, usize) {
    let (first_decimal, second_decimal, decimal_size) = equalize(first_decimal, second_decimal);
    assert_eq!(first_decimal.decimal_size, second_decimal.decimal_size);
    let (mut result, _) = first_decimal
        .numbers
        .iter()
        .rev()
        .zip(second_decimal.numbers.iter().rev())
        .fold((Vec::new(), 0), |(mut acc, carry), (&a, &b)| {
            let (result, carry) = compute(a, b, carry);
            acc.push(result);
            (acc, carry)
        });
    result.reverse();
    (result, decimal_size)
}
fn sum_with_carry(a: u8, b: u8, carry: u8) -> (u8, u8) {
    let sum = a + b + carry;
    let sum_digit = sum % 10;
    let carry = (sum - sum_digit) / 10;
    (sum_digit, carry)
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Decimal {
    type Output = Decimal;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Sign::Negative, _) => rhs - self.inverse(),
            (_, Sign::Negative) => self - rhs.inverse(),
            _ => {
                assert_eq!(Sign::Positive, self.sign);
                assert_eq!(Sign::Positive, rhs.sign);
                let (result, decimal_size) = accumulate(&self, &rhs, sum_with_carry);
                normalize(Decimal {
                    numbers: result,
                    decimal_size,
                    sign: Sign::Positive,
                })
            }
        }
    }
}
fn diff_with_borrow(a: u8, b: u8, borrow: u8) -> (u8, u8) {
    if a < (b + borrow) {
        (10 + a - (b + borrow), 1)
    } else {
        (a - (b + borrow), 0)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Decimal {
    type Output = Decimal;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (_, Sign::Negative) => self + rhs.inverse(),
            (_, _) if self < rhs => (rhs - self).inverse(),
            (_, _) => {
                assert_eq!(Sign::Positive, self.sign);
                assert_eq!(Sign::Positive, rhs.sign);
                assert!(self > rhs);
                let (result, decimal_size) = accumulate(&self, &rhs, diff_with_borrow);
                normalize(Decimal {
                    numbers: result,
                    decimal_size,
                    sign: Sign::Positive,
                })
            }
        }
    }
}
fn multiply_row_by(input: &[u8], multiplicand: u8) -> Vec<u8> {
    let carry = 0u8;
    let mul =
        iter::once(&PAD)
            .chain(input.iter())
            .fold((Vec::new(), carry), |(mut acc, carry), &a| {
                let mul = a * multiplicand + carry;
                let rest = mul % 10;
                let carry = (mul - rest) / 10;
                acc.push(mul);
                (acc, carry)
            });
    mul.0
}
fn add_all_rows(rows: &[Vec<u8>]) -> Decimal {
    rows.iter().fold(Decimal::default(), |acc, row| {
        acc + Decimal {
            numbers: row.clone(),
            decimal_size: 0,
            sign: Sign::Positive,
        }
    })
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Mul for Decimal {
    type Output = Decimal;
    fn mul(self, rhs: Self) -> Self::Output {
        let rows: Vec<Vec<u8>> = self
            .numbers
            .iter()
            .rev()
            .enumerate()
            .map(|(index, &multiplicand)| {
                let mut multiply_row = multiply_row_by(&rhs.numbers, multiplicand);
                multiply_row.extend(vec![0; index]);
                multiply_row
            })
            .collect();
        let acc = add_all_rows(&rows);
        normalize(Decimal {
            numbers: acc.numbers,
            decimal_size: self.decimal_size + rhs.decimal_size,
            sign: if self.sign == rhs.sign {
                Sign::Positive
            } else {
                Sign::Negative
            },
        })
    }
}