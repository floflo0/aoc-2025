use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day01/input.txt";

fn part1(input_file_path: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let rotations = &line[1..].parse::<i32>().unwrap();
        match line.chars().next().unwrap() {
            'L' => dial -= rotations,
            'R' => dial += rotations,
            _ => unreachable!(),
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn part2(input_file_path: &str) -> i32 {
    let mut dial: i32 = 50;
    let mut zero_count: i32 = 0;
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let rotations = &line[1..].parse::<i32>().unwrap();
        match line.chars().next().unwrap() {
            'L' => {
                let prev_dial = dial;
                dial -= rotations;
                if dial <= 0 {
                    zero_count += -(dial / 100);
                    if prev_dial != 0 {
                        zero_count += 1;
                    }
                }
            }
            'R' => {
                dial += rotations;
                if dial > 99 {
                    zero_count += dial / 100;
                }
            }
            _ => unreachable!(),
        }
        dial = dial.rem_euclid(100);
    }
    zero_count
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day01/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 6);
    }
}
