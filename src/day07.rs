use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day07/input.txt";

fn part1(input_file_path: &str) -> i32 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut positions: BTreeSet<usize> = BTreeSet::new();
    positions.insert(lines.next().unwrap().find("S").unwrap());
    let mut splits_count = 0;
    for line in lines {
        let old_positions: BTreeSet<usize> = positions.clone();
        positions.clear();
        for position in old_positions.into_iter() {
            if line.chars().nth(position).unwrap() == '^' {
                positions.insert(position - 1);
                positions.insert(position + 1);
                splits_count += 1;
            } else {
                positions.insert(position);
            }
        }
    }
    splits_count
}

#[derive(Clone)]
struct Timeline {
    position: usize,
    count: i64,
}

impl PartialEq for Timeline {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Timeline {}

impl PartialOrd for Timeline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timeline {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.position < other.position {
            return Ordering::Less;
        }

        if self.position > other.position {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

fn insert_timeline(timelines: &mut BTreeSet<Timeline>, timeline: &Timeline) {
    if timelines.contains(timeline) {
        let duplicate_timeline = timelines.get(timeline).unwrap();
        timelines.replace(Timeline {
            position: timeline.position,
            count: duplicate_timeline.count + timeline.count,
        });
        return;
    }
    timelines.insert(timeline.clone());
}

fn part2(input_file_path: &str) -> i64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut timelines: BTreeSet<Timeline> = BTreeSet::new();
    timelines.insert(Timeline {
        position: lines.next().unwrap().find("S").unwrap(),
        count: 1,
    });
    for line in lines {
        let old_timelines: BTreeSet<Timeline> = timelines.clone();
        timelines.clear();
        for timeline in old_timelines.iter() {
            if line.chars().nth(timeline.position).unwrap() == '^' {
                insert_timeline(
                    &mut timelines,
                    &Timeline {
                        position: timeline.position - 1,
                        count: timeline.count,
                    },
                );
                insert_timeline(
                    &mut timelines,
                    &Timeline {
                        position: timeline.position + 1,
                        count: timeline.count,
                    },
                );
            } else {
                insert_timeline(&mut timelines, timeline);
            }
        }
    }
    timelines
        .iter()
        .fold(0, |acc, timeline| acc + timeline.count)
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day07/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 40);
    }
}
