extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

/// # Remarks
/// see Modular_exponentiation[https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method]
fn modular_exponentiation(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        result = match exponent & 1 == 1 {
            true => (result.checked_mul(base).expect("overflow")) % modulus,
            false => result,
        };
        exponent >>= 1;
        base = (base.checked_mul(base).expect("overflow")) % modulus;
    }
    result
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}


pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}
