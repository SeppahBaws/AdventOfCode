fn main() {
    let lines = common::file::split_file_by_lines("day_01/input.txt");

    println!("Total count for part one: {}", part_one(&lines));
    println!("Total count for part two: {}", part_two(&lines));
}

fn part_one(lines: &Vec<String>) -> u32 {
    let mut count = 0;

    for line in lines {
        let digits = extract_digits(&line);
        let number = combine_digits(digits);
        count += number;
    }

    count
}

fn part_two(lines: &Vec<String>) -> u32 {
    let mut count = 0;

    for line in lines {
        let digits = extract_digits_spelled(&line);
        let number = combine_digits(digits);
        count += number;
    }

    count
}

fn extract_digits(line: &String) -> Vec<u32> {
    let digits = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| String::from(c).parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    debug_assert!(digits.len() > 0);

    digits
}

fn extract_digits_spelled(line: &String) -> Vec<u32> {
    let spelled: Vec<String> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .map(|c| c.to_string())
    .collect();

    let mut digits: Vec<u32> = vec![];

    let mut i = 0;
    while i < line.len() {
        let curr_char = line.chars().nth(i).unwrap();

        if curr_char.is_numeric() {
            // If the current char is numeric, simply use that
            digits.push(String::from(curr_char).parse::<u32>().unwrap());
        } else if let Some(found) = spelled.iter().find(|number| {
            line.len() >= number.len()
                && i <= line.len() - number.len()
                && &line[i..i + number.len()] == number.as_str()
        }) {
            // If we have a spelled number starting at the current index, check which digit it is and use that
            let found_pos = spelled.iter().position(|num| num == found).unwrap();
            digits.push(found_pos as u32 + 1);

            i += found.len();
            continue;
        }

        i += 1;
    }

    digits
}

fn combine_digits(chars: Vec<u32>) -> u32 {
    vec![
        chars.first().clone().unwrap().to_string(),
        chars.last().clone().unwrap().to_string(),
    ]
    .join("")
    .parse::<u32>()
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_digits() {
        assert_eq!(extract_digits(&String::from("1abc2")), vec![1, 2]);
        assert_eq!(extract_digits(&String::from("pqr3stu8vwx")), vec![3, 8]);
        assert_eq!(
            extract_digits(&String::from("a1b2c3d4e5f")),
            vec![1, 2, 3, 4, 5]
        );
        assert_eq!(extract_digits(&String::from("treb7uchet")), vec![7]);
    }

    #[test]
    fn collects_digits_spelled() {
        assert_eq!(
            extract_digits_spelled(&String::from("two1nine")),
            vec![2, 1, 9]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("eightwothree")),
            vec![8, 3]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("abcone2threexyz")),
            vec![1, 2, 3]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("xtwone3four")),
            vec![2, 3, 4]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("4nineeightseven2")),
            vec![4, 9, 8, 7, 2]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("zoneight234")),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("7pqrstsixteen")),
            vec![7, 6]
        );
        assert_eq!(
            extract_digits_spelled(&String::from("six97")),
            vec![6, 9, 7]
        );
        assert_eq!(extract_digits_spelled(&String::from("37")), vec![3, 7]);
        assert_eq!(extract_digits_spelled(&String::from("2s")), vec![2]);
        assert_eq!(
            extract_digits_spelled(&String::from(
                "qsqb6pfxxvrbnhc7sevenzdzrtkzhjmchnrbzksmkrvcx"
            )),
            vec![6, 7, 7]
        );
    }

    #[test]
    fn combines_digits() {
        assert_eq!(combine_digits(vec![1, 2]), 12);
        assert_eq!(combine_digits(vec![3, 8]), 38);
        assert_eq!(combine_digits(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(combine_digits(vec![7]), 77);

        assert_eq!(combine_digits(vec![2, 1, 9]), 29);
        assert_eq!(combine_digits(vec![8, 2, 3]), 83);
        assert_eq!(combine_digits(vec![1, 2, 3]), 13);
        assert_eq!(combine_digits(vec![2, 3, 4]), 24);
        assert_eq!(combine_digits(vec![4, 9, 8, 7, 2]), 42);
        assert_eq!(combine_digits(vec![1, 8, 2, 3, 4]), 14);
        assert_eq!(combine_digits(vec![7, 6]), 76);

        assert_ne!(combine_digits(vec![7]), 7);
    }
}
