use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day03/input.txt";

fn part1(input_file_path: &str) -> u32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let digits: Vec<u32> = line
                .chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect();
            let mut max_digit: u32 = digits[0];
            let mut max_digit_index = 0;
            for (i, digit) in digits.iter().enumerate().take(digits.len() - 1) {
                if *digit > max_digit {
                    max_digit = *digit;
                    max_digit_index = i;
                }
            }
            let second_digit = digits[(max_digit_index + 1)..].iter().max().unwrap();
            max_digit * 10 + second_digit
        })
        .sum()
}

fn larger_joltage(digits: &[u32], number_batteries: usize) -> u64 {
    let mut max_digit: u32 = digits[0];
    let mut max_digit_index = 0;

    for (i, digit) in digits
        .iter()
        .enumerate()
        .take(digits.len() - number_batteries + 1)
    {
        if *digit > max_digit {
            max_digit = *digit;
            max_digit_index = i;
        }
    }

    if number_batteries == 1 {
        return max_digit as u64;
    }

    max_digit as u64 * 10_u64.pow(number_batteries as u32 - 1)
        + larger_joltage(&digits[(max_digit_index + 1)..], number_batteries - 1)
}

fn part2(input_file_path: &str) -> u64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            larger_joltage(
                &line
                    .unwrap()
                    .chars()
                    .map(|digit| digit.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
                12,
            )
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day03/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 3121910778619);
    }
}
