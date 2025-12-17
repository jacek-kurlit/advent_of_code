use md5::{Digest, Md5};

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    find_counter(input, "00000")
}

fn find_counter(input: &str, prefix: &str) -> usize {
    let mut counter = 0;
    loop {
        let mut hasher = Md5::new();
        let value = format!("{input}{counter}");
        hasher.update(value.as_bytes());
        let hash = hasher.finalize();
        if base16ct::lower::encode_string(&hash).starts_with(prefix) {
            return counter;
        }
        counter += 1;
    }
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    find_counter(input, "000000")
}

#[cfg(test)]
mod tests {

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"abcdef";
        assert_eq!(part_1(input), 609043);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = "ckczppom";
        assert_eq!(part_1(input), 117946);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = "ckczppom";
        assert_eq!(part_2(input), 3938038);
    }
}
