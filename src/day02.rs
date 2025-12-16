use std::fs::File;
use std::io::Read;

const INPUT_FILE_PATH: &str = "assets/day02/input.txt";

fn part1(input_file_path: &str) -> i64 {
    let mut file = File::open(input_file_path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .for_each(|range| {
            let range = range.to_string();
            let (min, max) = range.split_once('-').unwrap();
            let min = min.parse::<i64>().unwrap();
            let max = max.parse::<i64>().unwrap();
            for number in min..=max {
                let number_string = number.to_string();
                if number_string.len() % 2 == 0 {
                    let middle = number_string.len() / 2;
                    if number_string[0..middle] == number_string[middle..] {
                        sum += number;
                    }
                }
            }
        });
    sum
}

fn has_pattern(number_string: String) -> bool {
    let number_string_length = number_string.len();
    for i in 1..number_string_length {
        if number_string_length.is_multiple_of(i) {
            let pattern = &number_string[0..i];
            let mut bad_pattern = true;
            for j in 1..(number_string_length / i) {
                if pattern != &number_string[(i * j)..(i * (j + 1))] {
                    bad_pattern = false;
                    break;
                }
            }
            if bad_pattern {
                return true;
            }
        }
    }
    false
}

fn part2(input_file_path: &str) -> i64 {
    let mut file = File::open(input_file_path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut sum = 0;
    input
        .strip_suffix('\n')
        .unwrap()
        .split(',')
        .for_each(|range| {
            let range = range.to_string();
            let (min, max) = range.split_once('-').unwrap();
            let min = min.parse::<i64>().unwrap();
            let max = max.parse::<i64>().unwrap();
            for number in min..=max {
                if has_pattern(number.to_string()) {
                    sum += number;
                }
            }
        });
    sum
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day02/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 4174379265);
    }
}
