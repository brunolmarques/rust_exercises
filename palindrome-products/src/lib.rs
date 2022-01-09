#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome();
impl Palindrome {
    pub fn new(_a: u64, _b: u64) -> Palindrome {
        Palindrome{}
    }
    pub fn value(&self) -> u64 {
        unimplemented!("return the value of this palindrome")
    }
    pub fn insert(&mut self, _a: u64, _b: u64) {
    }
}
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    return if max <= min || min + 1 == max {
        None
    } else {
        Some((Palindrome::new(min, max), Palindrome::new(min, max)))
    }
}
