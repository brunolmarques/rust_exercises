use std::collections::HashMap;

const PLAIN: &'static str = "abcdefghijklmnopqrstuvwxyz";
const CIPHER: &'static str = "zyxwvutsrqponmlkjihgfedcba";

fn switch(s: &str) -> Vec<String> {
    let cypher: HashMap<char, char> = PLAIN.chars().zip(CIPHER.chars()).collect();
    
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_digit(36))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().map(|&c| if cypher.contains_key(&c) { cypher.get(&c).unwrap().clone() } else { c }))
        .map(|chunk| chunk.collect::<String>())
        .collect::<Vec<String>>()
}

pub fn encode(s: &str) -> String {
    switch(s).join(" ")
}

pub fn decode(s: &str) -> String {
    switch(s).join("")
}

