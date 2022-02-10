/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let clean_isbn = str::replace(isbn, "-", "");

    if clean_isbn.len() != 10 { return false }

    clean_isbn
        .char_indices()
        .filter_map(digit_value)
        .zip((0..11).rev())
        .fold(0, |acc, (v, i)| acc + v * i) % 11 == 0

}

fn digit_value((index, c): (usize, char)) -> Option<u32> {
    if c == 'X' && index == 9 {
        Some(10)
    } else {
        c.to_digit(10)
    }
}