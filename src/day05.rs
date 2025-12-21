use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day05/input.txt";

fn part1(input_file_path: &str) -> i64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let range = line.split_once('-').unwrap();
        ranges.push((range.0.parse().unwrap(), range.1.parse().unwrap()));
    }

    lines
        .map(|line| line.parse::<i64>().unwrap())
        .fold(0, |acc, id| {
            if ranges.iter().any(|range| range.0 <= id && id <= range.1) {
                return acc + 1;
            }
            acc
        })
}

fn part2(input_file_path: &str) -> u64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let range = line.split_once('-').unwrap();
        let lower = range.0.parse::<u64>().unwrap();
        let upper = range.1.parse::<u64>().unwrap();
        debug_assert!(lower <= upper);
        let mut i = 0;
        let mut add_range = true;
        while i < ranges.len() {
            let range = ranges[i];
            if range.0 <= lower && lower <= range.1 {
                if upper <= range.1 {
                    add_range = false;
                    break;
                }

                let mut j = i + 1;
                let mut merged = false;
                while j < ranges.len() {
                    let range2 = ranges[j];
                    if range2.0 <= upper && upper <= range2.1 {
                        ranges[i].1 = range2.1;
                        ranges.remove(j);
                        merged = true;
                        break;
                    }
                    j += 1;
                }
                if !merged {
                    ranges[i].1 = upper;
                }
                add_range = false;
                break;
            }
            if range.0 <= upper && upper <= range.1 {
                let mut j = i + 1;
                let mut merged = false;
                while j < ranges.len() {
                    let range2 = ranges[j];
                    if range2.0 <= lower && lower <= range2.1 {
                        ranges[i].0 = range2.0;
                        ranges.remove(j);
                        merged = true;
                        break;
                    }
                    j += 1;
                }
                if !merged {
                    ranges[i].0 = lower;
                }
                add_range = false;
                break;
            }
            i += 1;
        }
        if add_range {
            ranges.push((lower, upper));
        }
    }
    ranges
        .iter()
        .enumerate()
        .filter(|(i, range)| {
            !ranges
                .iter()
                .enumerate()
                .any(|(j, range2)| range2.0 <= range.0 && range.1 <= range2.1 && *i != j)
        })
        .fold(0_u64, |acc, (_, range)| acc + range.1 - range.0 + 1)
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day05/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 14);
    }
}
