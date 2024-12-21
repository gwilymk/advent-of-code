use std::iter;

use aoc2024::{get_input, Direction, Vector2D};

fn main() {
    let input = get_input(21);
    println!("Part 1: {}", part1(&input)); // 295616 too high
}

fn keypad_coordinate(number: char) -> Vector2D<i32> {
    match number {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => panic!("Unknown button '{number}'"),
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DpadInstruction {
    A,
    Direction(Direction),
}

impl From<DpadInstruction> for char {
    fn from(value: DpadInstruction) -> Self {
        match value {
            DpadInstruction::A => 'A',
            DpadInstruction::Direction(direction) => match direction {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            },
        }
    }
}

fn dpad_coordinate(instr: DpadInstruction) -> Vector2D<i32> {
    match instr {
        DpadInstruction::A => (2, 0),
        DpadInstruction::Direction(direction) => match direction {
            Direction::North => (1, 0),
            Direction::East => (2, 1),
            Direction::South => (1, 1),
            Direction::West => (0, 1),
        },
    }
    .into()
}

fn keypad_sequence(code: &str) -> Vec<Vec<Vec<DpadInstruction>>> {
    let mut results = vec![];
    let mut current_coordinate = keypad_coordinate('A');

    for c in code.chars() {
        let new_coordinate = keypad_coordinate(c);
        let mut chunk = vec![];

        if new_coordinate.y != 2 && current_coordinate.y != 2 {
            // offer the other way too

            let mut result = vec![];

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::South),
                (new_coordinate.y - current_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::West),
                (current_coordinate.x - new_coordinate.x).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::North),
                (current_coordinate.y - new_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::East),
                (new_coordinate.x - current_coordinate.x).max(0) as usize,
            ));

            result.push(DpadInstruction::A);

            chunk.push(result);
        }

        {
            let mut result = vec![];

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::North),
                (current_coordinate.y - new_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::East),
                (new_coordinate.x - current_coordinate.x).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::South),
                (new_coordinate.y - current_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::West),
                (current_coordinate.x - new_coordinate.x).max(0) as usize,
            ));

            result.push(DpadInstruction::A);

            chunk.push(result);
        }

        results.push(chunk);
        current_coordinate = new_coordinate;
    }

    results
}

// goes right down first
fn dpad_sequence(sequence: &[DpadInstruction]) -> Vec<Vec<Vec<DpadInstruction>>> {
    let mut results = vec![];
    let mut current_coordinate = dpad_coordinate(DpadInstruction::A);

    for &instr in sequence {
        let new_coordinate = dpad_coordinate(instr);
        let mut chunk = vec![];

        if current_coordinate.x != 0 && new_coordinate.x != 0 {
            let mut result = vec![];

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::North),
                (current_coordinate.y - new_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::West),
                (current_coordinate.x - new_coordinate.x).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::East),
                (new_coordinate.x - current_coordinate.x).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::South),
                (new_coordinate.y - current_coordinate.y).max(0) as usize,
            ));

            result.push(DpadInstruction::A);

            chunk.push(result);
        }

        {
            let mut result = vec![];
            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::East),
                (new_coordinate.x - current_coordinate.x).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::South),
                (new_coordinate.y - current_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::North),
                (current_coordinate.y - new_coordinate.y).max(0) as usize,
            ));

            result.extend(iter::repeat_n(
                DpadInstruction::Direction(Direction::West),
                (current_coordinate.x - new_coordinate.x).max(0) as usize,
            ));

            result.push(DpadInstruction::A);

            chunk.push(result);
        }

        results.push(chunk);
        current_coordinate = new_coordinate;
    }

    results
}

fn part1_line(input: &str) -> String {
    let keypad_input = keypad_sequence(input);

    let dpad2 = keypad_input
        .iter()
        .flat_map(|section| {
            let dpad1_options = section.iter().map(|chunk| dpad_sequence(chunk));

            let dpad2_options = dpad1_options
                .flat_map(|dpad1_option| {
                    dpad1_option
                        .iter()
                        .flat_map(|dpad1_chunk_options| {
                            dpad1_chunk_options
                                .iter()
                                .map(|dpad1_chunk_option| {
                                    let dpad2_options = dpad_sequence(dpad1_chunk_option);
                                    dpad2_options
                                        .iter()
                                        .flat_map(|dpad2_options| {
                                            dpad2_options
                                                .iter()
                                                .min_by_key(|option| option.len())
                                                .unwrap()
                                        })
                                        .copied()
                                        .collect::<Vec<_>>()
                                })
                                .min_by_key(|dpad2_option| dpad2_option.len())
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            dpad2_options.into_iter().min_by_key(|o| o.len()).unwrap()
        })
        .collect::<Vec<_>>();

    println!("{input}");

    println!(
        "dpad2:  {}",
        dpad2.iter().copied().map(char::from).collect::<String>()
    );
    println!("==========");

    dpad2.into_iter().map(char::from).collect()
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line: &str| {
            let dpad2 = part1_line(input);

            let number = line[..line.len() - 1].parse::<usize>().unwrap();

            number * dpad2.len()
        })
        .sum()
}

#[test]
fn given_input() {
    assert_eq!(
        part1_line("029A").len(),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("980A").len(),
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
    );

    assert_eq!(
        part1_line("179A").len(),
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("456A").len(),
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("379A").len(),
        "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
    );

    assert_eq!(
        part1(
            "029A
980A
179A
456A
379A"
        ),
        126384
    );
}
