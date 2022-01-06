pub fn raindrops(x: u32) -> String {
    let is_factor = |factor| x % factor == 0;
    let mut rez = String::new();

    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }
    if rez.is_empty() { rez = x.to_string(); }

    rez
}
