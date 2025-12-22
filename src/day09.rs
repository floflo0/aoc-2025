use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day09/input.txt";

fn part1(input_file_path: &str) -> i64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let red_tiles: Vec<(i64, i64)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coord = line.split_once(',').unwrap();
            (coord.0.parse().unwrap(), coord.1.parse().unwrap())
        })
        .collect();
    let mut areas: Vec<i64> = Vec::with_capacity((red_tiles.len() * (red_tiles.len() - 1)) / 2);
    for (i, (x1, y1)) in red_tiles.iter().enumerate() {
        for (x2, y2) in red_tiles.iter().skip(i + 1) {
            areas.push(((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1));
        }
    }
    *areas.iter().max().unwrap()
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day09/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 50);
    }
}
