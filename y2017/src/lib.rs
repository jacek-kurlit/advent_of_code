pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        if true {
            println!("star");
        } else {
            println!("rst");
        }
        let x = vec![1, 2, 3];
        println!("{x:?}");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
