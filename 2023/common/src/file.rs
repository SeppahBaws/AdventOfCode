/// Reads a file from a path and returns a vector of lines in the file.
pub fn split_file_by_lines(path: &str) -> Vec<String> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");

    contents.lines().map(|s| String::from(s)).collect()
}
