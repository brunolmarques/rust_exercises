use std::collections::HashSet;

const CASE_MASK: u8 = b'a' ^ b'A';

#[inline(always)]
fn char_lowercase(c: char) -> char {
    if c.is_ascii_alphabetic() {
        (c as u8 | CASE_MASK) as char
    } else if c.is_uppercase() {
        c.to_lowercase().next().unwrap()
    } else {
        c
    }
}

#[inline(always)]
fn string_lowercase(s: &str) -> String {
    s.chars().map(|c| char_lowercase(c)).collect()
}

#[inline(always)]
fn checksum(word: &str) -> u8 {
    word.bytes().fold(0u8, |a, b| a.overflowing_add(b).0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = string_lowercase(&word);
    let w_cs = checksum(&word);
    let wlen = word.len();
    possible_anagrams
        .iter()
        .filter(|&pa| {
            let pa = string_lowercase(pa);
            if pa.len() != wlen || w_cs != checksum(&pa) || pa == word {
                return false;
            }
            let mut len: usize = 0;
            pa.chars().all(|c| {
                len += 1;
                len > wlen
                    || word.chars().filter(|&x| x == c).count()
                    == pa.chars().filter(|&x| x == c).count()
            })
        })
        .map(|&x| x)
        .collect()
}