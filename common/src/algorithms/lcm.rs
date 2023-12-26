use super::gcd::gcd;

pub fn lcm_of_n_numbers(numbers: &[u32]) -> u32 {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm_of_n_numbers(&numbers[1..]);
    a * b
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}
