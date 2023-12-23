pub mod matrix;
mod template;

pub fn load_input(path: &str) -> String {
    let file_name = format!(
        "{}/input/{}",
        std::env::current_dir().unwrap().display(),
        path
    );
    std::fs::read_to_string(&file_name)
        .unwrap_or_else(|_| panic!("Failed to read input file {}", file_name))
}
