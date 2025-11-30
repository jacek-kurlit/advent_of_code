use common::inputs::load_input_for_day;
use y2017::day1;

#[test]
fn solve_da1_task1_puzzle() {
    assert_eq!(day1::task1(&load_input_for_day(1)), 1228)
}

#[test]
fn solve_da1_task2_puzzle() {
    assert_eq!(day1::task2(&load_input_for_day(1)), 1238)
}
