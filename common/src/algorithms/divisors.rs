use std::collections::HashSet;

pub fn divisors(n: u64) -> Vec<u64> {
    let mut result = HashSet::new();
    let mut i = 1;
    while i * i <= n {
        if n.is_multiple_of(i) {
            result.insert(i);
            result.insert(n / i);
        }
        i += 1;
    }
    let mut sorted: Vec<u64> = result.into_iter().collect();
    sorted.sort_unstable();

    sorted
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm() {
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(2), vec![1, 2]);
        assert_eq!(divisors(3), vec![1, 3]);
        assert_eq!(divisors(4), vec![1, 2, 4]);
        assert_eq!(divisors(5), vec![1, 5]);
        assert_eq!(divisors(6), vec![1, 2, 3, 6]);
        assert_eq!(divisors(7), vec![1, 7]);
        assert_eq!(divisors(8), vec![1, 2, 4, 8]);
        assert_eq!(divisors(9), vec![1, 3, 9]);
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
    }
}
