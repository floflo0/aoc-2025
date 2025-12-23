use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day10/input.txt";

fn solve_part1(
    lights: u16,
    buttons: &Vec<u16>,
    map: &mut HashMap<u16, Option<i32>>,
) -> Option<i32> {
    if let Some(&value) = map.get(&lights) {
        return value;
    }
    map.insert(lights, None);
    let mut min: Option<i32> = None;
    for button in buttons {
        if let Some(value) = solve_part1(lights ^ button, buttons, map) {
            let value = value + 1;
            min = Some(min.map_or(value, |old_value| old_value.min(value)));
        }
    }
    if min.is_some() {
        map.insert(lights, min);
    }
    min
}

fn part1(input_file_path: &str) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut tokens = line.split_whitespace();
            let lights = tokens.next().unwrap().chars().collect::<Vec<char>>();
            let lights_length = lights.len();
            let target_lights: u16 = lights
                .into_iter()
                .skip(1)
                .take(lights_length - 2)
                .enumerate()
                .map(|(i, light)| if light == '#' { 1 << i } else { 0 })
                .sum();
            let buttons: Vec<u16> = tokens
                .filter(|token| token.starts_with('('))
                .map(|buttons| {
                    buttons[1..(buttons.len() - 1)]
                        .split(',')
                        .map(|button| 1 << button.parse::<u16>().unwrap())
                        .sum()
                })
                .collect();
            let mut map: HashMap<u16, Option<i32>> = HashMap::new();
            map.insert(target_lights, Some(0));
            solve_part1(0, &buttons, &mut map).unwrap()
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day10/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 7);
    }
}
