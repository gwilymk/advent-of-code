use std::collections::{HashMap, HashSet};

use aoc2025::Grid2;

fn main() {
    let input = aoc2025::get_input(7);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Start,
    Blank,
    Splitter,
}

fn part1(input: &str) -> usize {
    let grid = parse(input);

    let start_pos = grid.points[0]
        .iter()
        .position(|s| s == &State::Start)
        .expect("Should have start on the first row");

    let mut beams = HashSet::new();
    beams.insert(start_pos);

    let mut splits = 0;

    for y in 1..grid.height {
        let row = &grid.points[y];
        let mut new_beams = beams.clone();

        for (i, state) in row.iter().enumerate() {
            if state != &State::Splitter {
                continue;
            }

            if beams.contains(&i) {
                new_beams.remove(&i);
                new_beams.insert(i - 1);
                new_beams.insert(i + 1);
                splits += 1;
            }
        }

        beams = new_beams;
    }

    splits
}

fn part2(input: &str) -> usize {
    let grid = parse(input);

    let start_pos = grid.points[0]
        .iter()
        .position(|s| s == &State::Start)
        .expect("Should have start on the first row");

    let mut beams = HashMap::new();
    beams.insert(start_pos, 1usize);

    for y in 1..grid.height {
        let row = &grid.points[y];
        let mut new_beams = beams.clone();

        for (i, state) in row.iter().enumerate() {
            if state != &State::Splitter {
                continue;
            }

            if let Some(current_value) = new_beams.remove(&i) {
                *new_beams.entry(i - 1).or_default() += current_value;
                *new_beams.entry(i + 1).or_default() += current_value;
            }
        }

        beams = new_beams;
    }

    beams.values().sum()
}

fn parse(input: &str) -> Grid2<State> {
    Grid2::parse(input, |line| {
        line.chars()
            .map(|c| match c {
                '.' => State::Blank,
                '^' => State::Splitter,
                'S' => State::Start,
                _ => unreachable!(),
            })
            .collect()
    })
}

#[cfg(test)]
const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), 40);
}
