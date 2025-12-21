use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "assets/day08/input.txt";

#[derive(Eq, PartialEq)]
struct JunctionBoxe {
    x: i32,
    y: i32,
    z: i32,
}

impl Hash for JunctionBoxe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

#[derive(Eq)]
struct Connection<'a> {
    distance: i64,
    boxe1: &'a JunctionBoxe,
    boxe2: &'a JunctionBoxe,
}

impl<'a> PartialEq for Connection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl<'a> PartialOrd for Connection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Connection<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn part1(input_file_path: &str, number_connections: usize) -> usize {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let junction_boxes: Vec<JunctionBoxe> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coords = line.split(',').collect::<Vec<&str>>();
            JunctionBoxe {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
                z: coords[2].parse().unwrap(),
            }
        })
        .collect();
    let mut set: BTreeSet<Connection> = BTreeSet::new();
    for (i, boxe1) in junction_boxes.iter().enumerate() {
        for boxe2 in junction_boxes.iter().skip(i + 1) {
            set.insert(Connection {
                distance: (boxe1.x as i64 - boxe2.x as i64).pow(2)
                    + (boxe1.y as i64 - boxe2.y as i64).pow(2)
                    + (boxe1.z as i64 - boxe2.z as i64).pow(2),
                boxe1,
                boxe2,
            });
        }
    }
    let mut circuits: Vec<HashSet<&JunctionBoxe>> = set
        .iter()
        .take(number_connections)
        .map(|connection| {
            let mut set: HashSet<&JunctionBoxe> = HashSet::new();
            set.insert(connection.boxe1);
            set.insert(connection.boxe2);
            set
        })
        .collect();

    let mut i = 0;
    while i < circuits.len() {
        let mut changed = true;
        while changed {
            changed = false;
            for j in (i + 1)..(circuits.len()) {
                if !circuits[i].is_disjoint(&circuits[j]) {
                    let set = circuits.remove(j);
                    circuits[i].extend(set);
                    changed = true;
                    break;
                }
            }
        }
        i += 1;
    }

    let mut circuits = circuits
        .iter()
        .take(number_connections)
        .map(|set| set.len())
        .collect::<Vec<usize>>();
    circuits.sort();
    circuits.iter().rev().take(3).product()
}

fn merge_circuits(circuits: &mut Vec<HashSet<&JunctionBoxe>>) {
    let mut i = 0;
    while i < circuits.len() {
        let mut changed = true;
        while changed {
            changed = false;
            for j in (i + 1)..(circuits.len()) {
                if !circuits[i].is_disjoint(&circuits[j]) {
                    let set = circuits.remove(j);
                    circuits[i].extend(set);
                    changed = true;
                    break;
                }
            }
        }
        i += 1;
    }
}

fn part2(input_file_path: &str) -> i64 {
    let file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(file);
    let junction_boxes: Vec<JunctionBoxe> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let coords = line.split(',').collect::<Vec<&str>>();
            JunctionBoxe {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
                z: coords[2].parse().unwrap(),
            }
        })
        .collect();
    let mut set: BTreeSet<Connection> = BTreeSet::new();
    for (i, boxe1) in junction_boxes.iter().enumerate() {
        for boxe2 in junction_boxes.iter().skip(i + 1) {
            set.insert(Connection {
                distance: (boxe1.x as i64 - boxe2.x as i64).pow(2)
                    + (boxe1.y as i64 - boxe2.y as i64).pow(2)
                    + (boxe1.z as i64 - boxe2.z as i64).pow(2),
                boxe1,
                boxe2,
            });
        }
    }

    let mut connections_iter = set.iter();
    let mut circuits: Vec<HashSet<&JunctionBoxe>> = Vec::new();
    let mut connection: &Connection;
    loop {
        let mut circuit: HashSet<&JunctionBoxe> = HashSet::new();
        connection = connections_iter.next().unwrap();
        circuit.insert(connection.boxe1);
        circuit.insert(connection.boxe2);
        circuits.push(circuit);
        merge_circuits(&mut circuits);
        if circuits[0].len() == junction_boxes.len() {
            break;
        }
    }

    connection.boxe1.x as i64 * connection.boxe2.x as i64
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE_PATH, 1000));
    println!("Part 2: {}", part2(INPUT_FILE_PATH));
}

#[cfg(test)]
pub mod test {
    use super::*;

    const EXAMPLE_FILE_PATH: &str = "assets/day08/example.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_FILE_PATH, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_FILE_PATH), 25272);
    }
}
