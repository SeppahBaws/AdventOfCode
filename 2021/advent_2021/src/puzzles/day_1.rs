use anyhow::Result;

fn get_input() -> Result<Vec<u32>> {
    let input = common::file::read_input_file("advent_2021", 1)?;
    let input: Vec<u32> = input
        .split("\r\n")
        .map(|line| match line.parse::<u32>() {
            Ok(val) => val,
            Err(what) => panic!("Couldn't parse: {} - with string '{}'", what, line),
        })
        .collect();

    Ok(input)
}

fn part_1(input: &Vec<u32>) -> u32 {
    let mut amount_of_depth_increases = 0;
    for i in 1..input.len() {
        let item = input[i];
        if item > input[i - 1] {
            amount_of_depth_increases += 1;
        }
    }

    amount_of_depth_increases
}

fn part_2(input: &Vec<u32>) -> u32 {
    let mut amount_of_depth_increases = 0;
    for i in 1..(input.len() - 2) {
        let current_sum = input[i] + input[i + 1] + input[i + 2];
        let prev_sum = input[i - 1] + input[i] + input[i + 1];

        if current_sum > prev_sum {
            amount_of_depth_increases += 1;
        }
    }

    amount_of_depth_increases
}

pub fn execute() -> Result<()> {
    let input = get_input()?;

    let result_part_1 = part_1(&input);
    let result_part_2 = part_2(&input);

    print!("\n-----[ Part 1 ]-----\n");
    println!("Amount of depth increases: {}", result_part_1);

    print!("\n-----[ Part 2 ]-----\n");
    println!("Amount of depth increases: {}", result_part_2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{get_input, part_1, part_2};

    #[test]
    fn day_1_part_1() {
        let input = get_input().expect("Failed to load input from file");
        let result = part_1(&input);

        assert_eq!(result, 1696);
    }

    #[test]
    fn day_1_part_2() {
        let input = get_input().expect("Failed to load input from file");
        let result = part_2(&input);

        assert_eq!(result, 1737);
    }
}
