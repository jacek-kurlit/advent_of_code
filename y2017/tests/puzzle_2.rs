use y2017::day2;

#[test]
fn should_solve_puzzle1() {
    let input = common::inputs::load_input_for_day(2);
    assert_eq!(day2::task1(input), 32020);
}

#[test]
fn should_solve_puzzle2() {
    let input = common::inputs::load_input_for_day(2);
    assert_eq!(day2::task2(input), 236);
}
