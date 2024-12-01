fn main() {
    let lines = common::file::split_file_by_lines("day_02/input.txt");

    println!("Total count for part one: {}", part_one(&lines));
    // println!("Total count for part two: {}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    for line in lines {
        
        let a: Vec<&str> = line.split(": ").collect();
    }

    0
}
