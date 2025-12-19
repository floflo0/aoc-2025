use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day04/input.txt";

fn part1(input_file_path: &str) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut accessible_paper_rolls = 0;
    let map_width = map[0].len() as isize;
    let map_height = map.len() as isize;
    for y in 0..map_height {
        for x in 0..map_width {
            if map[y as usize][x as usize] != '@' {
                continue;
            }

            let mut paper_rolls = 0;
            for y2 in max(y - 1, 0)..=min(y + 1, map_height - 1) {
                for x2 in max(x - 1, 0)..=min(x + 1, map_width - 1) {
                    if y == y2 && x == x2 {
                        continue;
                    }

                    if map[y2 as usize][x2 as usize] == '@' {
                        paper_rolls += 1;
                    }
                }
            }
            if paper_rolls < 4 {
                accessible_paper_rolls += 1;
            }
        }
    }
    accessible_paper_rolls
}

fn part2(input_file_path: &str) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut prev_accessible_paper_rolls = 1;
    let mut accessible_paper_rolls = 0;
    while accessible_paper_rolls != prev_accessible_paper_rolls {
        prev_accessible_paper_rolls = accessible_paper_rolls;
        let map_width = map[0].len() as isize;
        let map_height = map.len() as isize;
        for y in 0..map_height {
            for x in 0..map_width {
                if map[y as usize][x as usize] != '@' {
                    continue;
                }

                let mut paper_rolls = 0;
                for y2 in max(y - 1, 0)..=min(y + 1, map_height - 1) {
                    for x2 in max(x - 1, 0)..=min(x + 1, map_width - 1) {
                        if y == y2 && x == x2 {
                            continue;
                        }

                        if map[y2 as usize][x2 as usize] == '@' {
                            paper_rolls += 1;
                        }
                    }
                }
                if paper_rolls < 4 {
                    map[y as usize][x as usize] = '.';
                    accessible_paper_rolls += 1;
                }
            }
        }
    }
    accessible_paper_rolls
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day04/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 43);
    }
}
