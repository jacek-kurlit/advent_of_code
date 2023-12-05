pub fn task1(n: i32) -> u32 {
    if n == 1 {
        return 0;
    }
    let mut size: i32 = 3;
    while n > size.pow(2) {
        size += 2;
    }
    let size_pow2 = size.pow(2);
    let right_bot_val = size_pow2;
    let left_bot_val = size_pow2 - size + 1;
    let left_top_val = size_pow2 - 2 * size + 2;
    let right_top_val = size_pow2 - 3 * size + 3;
    let n_pos = if n >= left_bot_val {
        (size - right_bot_val + n, size)
    } else if n >= left_top_val {
        (1, size - left_bot_val + n)
    } else if n >= right_top_val {
        (size - left_top_val + n, 1)
    } else {
        (size, size - right_top_val + n)
    };
    let center_pos = ((size + 1) / 2, (size + 1) / 2);

    center_pos.0.abs_diff(n_pos.0) + center_pos.1.abs_diff(n_pos.1)
}

pub fn task2(n: i32) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_solve_task1() {
        assert_eq!(task1(23), 2);
        assert_eq!(task1(1024), 31);
    }

    #[test]
    fn should_solve_task2() {
        assert_eq!(task2(5), 10);
        assert_eq!(task2(23), 25);
        assert_eq!(task2(144), 304);
        assert_eq!(task2(747), 806);
    }
}
