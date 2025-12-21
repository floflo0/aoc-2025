use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day06/input.txt";

fn part1(input_file_path: &str) -> i64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|number| number.to_string())
                .collect()
        })
        .collect();
    let opetations = lines.last().unwrap();
    lines
        .iter()
        .take(lines.len() - 1)
        .map(|line| {
            line.iter()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .reduce(|acc, line| {
            acc.iter()
                .enumerate()
                .map(|(i, number)| match opetations[i].as_str() {
                    "+" => *number + line[i],
                    "*" => *number * line[i],
                    operation => panic!("unexpected opetation '{}'", operation),
                })
                .collect()
        })
        .unwrap()
        .iter()
        .sum()
}

fn part2(input_file_path: &str) -> u64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let digits = lines.len() - 1;
    let operations_line = lines.last().unwrap();
    let chars: Vec<char> = operations_line.chars().collect();

    let mut i = 0;
    let mut sum: u64 = 0;

    while i < chars.len() {
        let start = i;
        let operation = operations_line.chars().nth(i).unwrap();
        i += 1;
        while i < chars.len() && chars[i] == ' ' {
            i += 1;
        }
        if i == chars.len() {
            i += 1;
        }
        let end = i - 1;
        let numbers: Vec<u64> = (start..end)
            .map(|j| {
                let mut number: u64 = 0;
                for line in lines.iter().take(digits) {
                    let chr = line.chars().nth(j).unwrap();
                    if chr != ' ' {
                        number = number * 10 + chr.to_digit(10).unwrap() as u64
                    }
                }
                number
            })
            .collect();
        sum += match operation {
            '+' => numbers.into_iter().sum::<u64>(),
            '*' => numbers
                .into_iter()
                .reduce(|acc, number| acc * number)
                .unwrap(),
            _ => panic!("unexpected opetation '{}'", operation),
        };
    }
    sum
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day06/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 3263827);
    }
}
