use y2017::day2;

#[test]
fn should_solve_puzzle1() {
    let input = common::load_input("input2.txt");
    assert_eq!(day2::task1(input), 32020);
}
