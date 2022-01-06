use std::collections::HashMap;
pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let exps = input.split("==").collect::<Vec<&str>>();
    if exps.len() < 2 {
        return None;
    }
    let cond = read_exp(HashMap::new(), String::from(exps[0]), true);
    let cond = read_exp(cond, String::from(exps[1]), false);
    let (v, success) = blute_force(Vec::new(), &cond);
    match success {
        true => Some(cond.keys().zip(&v).map(|(x,y)| (*x,*y)).collect()),
        false => None,
    }
}
fn blute_force(mut v: Vec<u8>, a: &HashMap<char, Alphabet>) -> (Vec<u8>, bool) {
    if v.len() >= a.len() {
        let mut sum = 0;
        for (alpha, val) in a.values().zip(&v) {
            if alpha.not_zero && *val == 0 {
                return (v,false);
            }
            sum += alpha.coefficient * (*val as i64)
        }
        return (v, sum == 0);
    }
    for i in 0..=9 {
        if v.contains(&i) {
            continue;
        }
        v.push(i);

        let result = blute_force(v, a);
        if result.1 {
            return result
        }
        v = result.0;
        v.pop();
    }
    return (v, false);
}
fn read_exp(hmap: HashMap<char, Alphabet>, exp: String, left: bool) -> HashMap<char, Alphabet> {
    exp.split("+")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .fold(hmap, |mut emap, x| {
            for (i, c) in x.chars().rev().enumerate() {
                let mut num = 10i64.pow(i as u32);
                if !left {
                    num *= -1
                }
                match emap.get_mut(&c) {
                    Some(alphabet) => {
                        alphabet.coefficient += num;
                        if x.len() - 1 == i {
                            alphabet.not_zero = true;
                        }
                    },
                    None => {
                        emap.insert(c, Alphabet{coefficient:num, not_zero: (x.len() -1 == i)});
                    }
                }
            }
            emap
        })
}
#[derive(Debug)]
struct Alphabet {
    coefficient: i64,
    not_zero: bool,
}
