use std::fs::read_to_string;

pub fn load_input_for_day(day: u8) -> String {
    read_to_string(format!("input/day{}.txt", day)).expect("Failed to read input file")
}
