#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::cmp::PartialOrd>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    match (is_sub(_first_list, _second_list), is_sub(_second_list, _first_list)) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
fn is_sub<T: PartialEq>(sub: &[T], sup: &[T]) -> bool {
    if sup.len() < sub.len() {
        return false;
    }
    for offset in 0..=sup.len() - sub.len() {
        if sub.iter().zip(sup[offset..].iter()).all(|(a, b)| a == b) {
            return true;
        }
    }
    false
}

