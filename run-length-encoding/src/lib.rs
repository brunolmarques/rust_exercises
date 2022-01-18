use std::iter;

pub fn encode(source: &str) -> String {
    let mut code = String::new();
    let mut remainder = source;

    while let Some(c) = remainder.chars().next() {
        let count = remainder.chars().take_while(|&next| next == c).count();

        match count {
            1 => code.push(c),
            _ => code.push_str(&format!("{}{}", count, c)),
        }

        remainder = &remainder[count..];
    }
    code
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .filter(|&c| !c.is_numeric())
        .zip(
            source
                .split(|c: char| !c.is_numeric())
                .map(|num| num.parse::<usize>().unwrap_or(1)),
        )
        .flat_map(|(c, count)| iter::repeat(c).take(count))
        .collect()
}
