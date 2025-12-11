use std::{
    collections::{HashMap, VecDeque},
    ops::Add,
};

use itertools::Itertools;

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    let devices = parse_input(input);
    let mut queue = VecDeque::new();
    queue.push_back("you");
    let mut count = 0;
    while let Some(key) = queue.pop_front() {
        for n in devices.get(key).unwrap() {
            if n == "out" {
                count += 1;
            } else {
                queue.push_back(n);
            }
        }
    }
    count
}

fn parse_input(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|l| {
            let (key, values) = l.split_once(": ").unwrap();
            (
                key.to_string(),
                values
                    .split_whitespace()
                    .map(|v| v.to_string())
                    .collect_vec(),
            )
        })
        .collect()
}

#[allow(dead_code)]
fn part_2(input: &str) -> u64 {
    let devices = parse_input(input);
    let mut memo = HashMap::new();
    let last_state = dfs("svr", &devices, &mut memo);
    dbg!(last_state);
    last_state.out
}

fn dfs(
    node: &str,
    devices: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, DfsState>,
) -> DfsState {
    if node == "out" {
        return DfsState {
            out: 1,
            ..Default::default()
        };
    }
    let mut fft = 0;
    if node == "fft" {
        fft = 1;
    }
    let mut dac = 0;
    if node == "dac" {
        dac = 1;
    }
    if let Some(cached) = memo.get(node) {
        return *cached;
    }
    let new_states = devices
        .get(node)
        .unwrap()
        .iter()
        .map(|next_node| dfs(next_node, devices, memo))
        .collect_vec();
    let mut new_state = calculate_new_state(new_states);
    new_state.fft = fft.max(new_state.fft);
    new_state.dac = dac.max(new_state.dac);

    memo.insert(node.to_string(), new_state);
    new_state
}

fn calculate_new_state(new_states: Vec<DfsState>) -> DfsState {
    let perfect_path = new_states
        .iter()
        .filter(|st| st.fft != 0 && st.dac != 0)
        .collect_vec();
    if !perfect_path.is_empty() {
        return perfect_path
            .into_iter()
            .copied()
            .reduce(|acc, v| acc + v)
            .unwrap();
    }
    let dac_path = new_states
        .iter()
        .filter(|st| st.fft == 0 && st.dac != 0)
        .collect_vec();
    if !dac_path.is_empty() {
        return dac_path
            .into_iter()
            .copied()
            .reduce(|acc, v| acc + v)
            .unwrap();
    }
    new_states.into_iter().reduce(|acc, v| acc + v).unwrap()
}

#[derive(Debug, Default, Clone, Copy)]
struct DfsState {
    out: u64,
    fft: u64,
    dac: u64,
}

impl Add for DfsState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            out: self.out + rhs.out,
            fft: self.fft.max(rhs.fft),
            dac: self.dac.max(rhs.dac),
        }
    }
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part_1(input), 5);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(11);
        assert_eq!(part_1(&input), 724);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part_2(input), 2);

        let input = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc fft
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part_2(input), 4);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(11);
        assert_eq!(part_2(&input), 473930047491888);
    }
}
