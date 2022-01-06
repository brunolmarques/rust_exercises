pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().chars().count() as u32;

    num.to_string().chars().try_fold(0, |acc, value |{
        value.to_digit(10)
            .map(|x| u32::pow(x,num_len))
            .map(|x| acc + x)
    }).map_or(false, |acc| acc == num)
}
