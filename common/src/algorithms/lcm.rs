use std::ops::{Div, Mul, Rem};

use super::gcd::gcd;

pub fn lcm_of_n_numbers<T>(numbers: &[T]) -> T
where
    T: Eq + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + From<u8> + Copy,
{
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm_of_n_numbers(&numbers[1..]);
    (a * b) / gcd(a, b)
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Eq + Mul<Output = T> + Div<Output = T> + Rem<Output = T> + From<u8> + Copy,
{
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn lcm_works() {
        assert_eq!(super::lcm(2, 3), 6);
        assert_eq!(super::lcm(5, 15), 15);
        assert_eq!(super::lcm(20, 50), 100);
        assert_eq!(super::lcm(3, 11), 33);
    }

    #[test]
    fn lcm_of_n_numbers_works() {
        assert_eq!(super::lcm_of_n_numbers(&[2, 3, 6]), 6);
        assert_eq!(super::lcm_of_n_numbers(&[2, 3, 5]), 30);
        assert_eq!(super::lcm_of_n_numbers(&[2, 3, 5, 7]), 210);
    }
}
