use anyhow::Result;
use std::{fs::File, io::Read};

pub fn read_input_file(project_path: &str, day: u32) -> Result<String> {
    let mut file = File::open(format!("./{}/input/day{}.txt", project_path, day))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}
