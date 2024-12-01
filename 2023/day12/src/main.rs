use rayon::prelude::*;
use std::{collections::HashMap, fmt, iter};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input.lines().map(arrangements_for_line).sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(arrangements_for_longer_line)
        .sum::<usize>()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SpringInformation {
    Operational,
    Damaged,
    MissingInformation,
}

impl fmt::Debug for SpringInformation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operational => write!(f, "."),
            Self::Damaged => write!(f, "#"),
            Self::MissingInformation => write!(f, "?"),
        }
    }
}

fn working_arrangements<'a>(
    spring_information: &'a [SpringInformation],
    contiguous_damaged: &'a [usize],
    arrangement_cache: &mut HashMap<(&'a [SpringInformation], &'a [usize]), usize>,
) -> usize {
    if let Some(&arrangements) = arrangement_cache.get(&(spring_information, contiguous_damaged)) {
        return arrangements;
    }

    if contiguous_damaged.is_empty() {
        return if spring_information.contains(&SpringInformation::Damaged) {
            0
        } else {
            1
        };
    }

    let mut start_position = 0;
    let mut total_arragements = 0;

    let number_damaged = contiguous_damaged[0];

    if spring_information.len() < number_damaged {
        // not enough space left for the number of damaged ones
        return 0;
    }

    // try to fit the first value into the spring information from start_position
    while start_position <= spring_information.len() - number_damaged {
        if spring_information[start_position..(start_position + number_damaged)]
            .contains(&SpringInformation::Operational)
        {
            if spring_information[start_position] == SpringInformation::Damaged {
                // this one was forced
                break;
            }

            start_position += 1;
            continue;
        }

        if spring_information.get(start_position + number_damaged)
            == Some(&SpringInformation::Damaged)
        {
            // we have more than number_damaged in a row if the first one here was forced to be damage from the start
            if spring_information[start_position] == SpringInformation::Damaged {
                break;
            } else {
                start_position += 1;
            }
        } else {
            let remaining_information =
                if spring_information.len() == start_position + number_damaged {
                    &[]
                } else {
                    &spring_information[start_position + number_damaged + 1..]
                };

            total_arragements += working_arrangements(
                remaining_information,
                &contiguous_damaged[1..],
                arrangement_cache,
            );

            // if the first value was forced to be damaged, then we can't continue searching
            if spring_information[start_position] == SpringInformation::Damaged {
                break;
            }

            start_position += 1;
        }
    }

    arrangement_cache.insert((spring_information, contiguous_damaged), total_arragements);

    total_arragements
}

fn parse(input: &str) -> (Vec<SpringInformation>, Vec<usize>) {
    let (map, condition_records) = input.split_once(' ').unwrap();
    (
        map.chars()
            .map(|c| match c {
                '.' => SpringInformation::Operational,
                '#' => SpringInformation::Damaged,
                '?' => SpringInformation::MissingInformation,
                _ => panic!("Unknown character {c}"),
            })
            .collect(),
        condition_records
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect(),
    )
}

fn arrangements_for_line(input: &str) -> usize {
    let (map, condition_records) = parse(input);
    let mut cache = HashMap::new();

    working_arrangements(&map, &condition_records, &mut cache)
}

fn arrangements_for_longer_line(input: &str) -> usize {
    let (map, condition_records) = parse(input);

    let map = iter::once(map)
        .cycle()
        .take(5)
        .collect::<Vec<_>>()
        .join(&SpringInformation::MissingInformation);
    let condition_records = condition_records.repeat(5);

    let mut cache = HashMap::new();

    working_arrangements(&map, &condition_records, &mut cache)
}

#[test]
fn check_given_input() {
    assert_eq!(arrangements_for_line("???.### 1,1,3"), 1);
    assert_eq!(arrangements_for_line(".??..??...?##. 1,1,3"), 4);
    assert_eq!(arrangements_for_line("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(arrangements_for_line("????.######..#####. 1,6,5"), 4);
    assert_eq!(arrangements_for_line("?###???????? 3,2,1"), 10);

    assert_eq!(arrangements_for_line("??????#??? 7,1"), 3);
    assert_eq!(arrangements_for_line("?#####???????#.. 5,1,1,1"), 6);
    assert_eq!(arrangements_for_line("???#??#.##. 2,1,2"), 2);
}

#[test]
fn check_longer_arrangements() {
    assert_eq!(arrangements_for_longer_line("???.### 1,1,3"), 1);
    assert_eq!(arrangements_for_longer_line(".??..??...?##. 1,1,3"), 16384);
    assert_eq!(arrangements_for_longer_line("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
    assert_eq!(arrangements_for_longer_line("????.#...#... 4,1,1"), 16);
    assert_eq!(
        arrangements_for_longer_line("????.######..#####. 1,6,5"),
        2500
    );
    assert_eq!(arrangements_for_longer_line("?###???????? 3,2,1"), 506250);
}
