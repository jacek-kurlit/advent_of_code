pub fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn rotate_right<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    transpose2(v)
        .into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transpose2() {
        // 1 2 3
        // 4 5 6
        //
        // 1 4
        // 2 5
        // 3 6
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(super::transpose2(v), expected);
    }

    #[test]
    fn test_rotate_right() {
        // 1 2 3
        // 4 5 6
        //
        // 4 1
        // 5 2
        // 6 3
        let v = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![4, 1], vec![5, 2], vec![6, 3]];
        assert_eq!(super::rotate_right(v), expected);
    }
}
