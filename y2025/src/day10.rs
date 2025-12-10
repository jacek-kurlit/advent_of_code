use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[allow(unused)]
struct Machine {
    target_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

#[allow(dead_code)]
fn part_1(input: &str) -> usize {
    parse_input(input).into_iter().map(find_steps_for).sum()
}

fn find_steps_for(machine: Machine) -> usize {
    let buttons = machine.buttons;
    let target_state = machine.target_state;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let init_state = vec![false; target_state.len()];
    seen.insert(init_state.clone());
    queue.push_back((vec![false; target_state.len()], 0));
    while let Some((current_state, steps)) = queue.pop_front() {
        if current_state == target_state {
            return steps;
        }
        for btns in &buttons {
            let new_state = create_next_state(&current_state, btns);
            if !seen.contains(&new_state) {
                seen.insert(new_state.clone());
                queue.push_back((new_state, steps + 1));
            }
        }
    }
    0
}

fn create_next_state(input_state: &[bool], buttons: &[usize]) -> Vec<bool> {
    let mut new_state = input_state.iter().copied().collect_vec();
    for button in buttons {
        new_state[*button] = !input_state[*button];
    }
    new_state
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(parse_machine).collect_vec()
}

fn parse_machine(line: &str) -> Machine {
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    let (states, rest) = line.split_once("]").unwrap();
    let target_states = states
        .trim()
        .chars()
        .skip(1)
        .map(|c| c == '#')
        .collect_vec();
    let (buttons_input, joltage_input) = rest.trim().split_once("{").unwrap();
    Machine {
        target_state: target_states,
        buttons: parse_buttons(buttons_input),
        joltage: parse_joltage(joltage_input),
    }
}

fn parse_buttons(input: &str) -> Vec<Vec<usize>> {
    input
        .split_whitespace()
        .map(|b| b.replace("(", "").replace(")", ""))
        .map(|b| b.split(",").map(|v| v.parse().unwrap()).collect_vec())
        .collect_vec()
}

fn parse_joltage(input: &str) -> Vec<usize> {
    input
        .trim()
        .replace("}", "")
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect_vec()
}

#[allow(dead_code)]
fn part_2(input: &str) -> usize {
    //this is the way
    //https://github.com/wilkotom/AdventOfCode/blob/main/rust/2025/day10/src/main.rs
    input.len()
}

#[cfg(test)]
mod tests {
    use common::inputs::load_input_for_day;

    use super::{part_1, part_2};

    #[test]
    fn solve_part_1_example() {
        let input = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part_1(input), 7);
    }

    #[test]
    fn solve_part_1_challenge() {
        let input = load_input_for_day(10);
        assert_eq!(part_1(&input), 425);
    }

    #[test]
    fn solve_part_2_example() {
        let input = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part_2(input), 33);
    }

    #[test]
    fn solve_part_2_challenge() {
        let input = load_input_for_day(10);
        assert_eq!(part_2(&input), 0);
    }
}
