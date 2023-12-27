use std::ops::Rem;

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Eq + Rem<Output = T> + From<u8> + Copy,
{
    if b == T::from(0) {
        return a;
    }
    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd_works() {
        assert_eq!(super::gcd(2, 3), 1);
        assert_eq!(super::gcd(2, 4), 2);
        assert_eq!(super::gcd(5, 20), 5);
        assert_eq!(super::gcd(7, 30), 1);
    }
}
