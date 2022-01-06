pub fn series(digits: &str, len: usize) -> Vec<String> {
    match (digits.len(), len) {
        (x, 0) => vec![String::from(""); x+1],
        (x, y) if y > x => vec![],
        (x, y) if x >= y => digits.chars()
            .collect::<Vec<char>>()
            .windows(y)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>(),
        (_, _) => vec![]
    }
}

