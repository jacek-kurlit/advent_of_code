use common::load_input;
use y2017::day1;

#[test]
fn solve_da1_task1_puzzle() {
    assert_eq!(day1::task1(&load_input("day1.txt")), 1228)
}

#[test]
fn solve_da1_task2_puzzle() {
    assert_eq!(day1::task2(&load_input("day1.txt")), 1238)
}
