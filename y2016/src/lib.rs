use common;

pub fn x() -> usize {
    common::add(1, 1)
}

pub fn y() -> usize {
    common::sub(1, 1)
}
#[cfg(test)]
mod test {
    #[test]
    fn ok() {
        assert_eq!(2, crate::x())
    }

    #[test]
    fn ok_sub() {
        assert_eq!(0, crate::y())
    }
}
