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

// prefers going up and right first
fn keypad_sequence(code: &str) -> Vec<DpadInstruction> {
    let mut result = vec![];
    let mut current_coordinate = keypad_coordinate('A');

    for c in code.chars() {
        let new_coordinate = keypad_coordinate(c);

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

        current_coordinate = new_coordinate;
    }

    result
}

// goes right down first
fn dpad_sequence(sequence: &[DpadInstruction]) -> Vec<DpadInstruction> {
    let mut result = vec![];
    let mut current_coordinate = dpad_coordinate(DpadInstruction::A);

    for &instr in sequence {
        let new_coordinate = dpad_coordinate(instr);

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
        current_coordinate = new_coordinate;
    }

    result
}

fn part1_line(input: &str) -> String {
    let keypad_input = keypad_sequence(input);
    let dpad1 = dpad_sequence(&keypad_input);
    let dpad2 = dpad_sequence(&dpad1);

    dpad2.into_iter().map(char::from).collect()
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| {
            let keypad_input = keypad_sequence(line);
            let dpad1 = dpad_sequence(&keypad_input);
            let dpad2 = dpad_sequence(&dpad1);

            let number = line[..line.len() - 1].parse::<usize>().unwrap();

            number * dpad2.len()
        })
        .sum()
}

#[test]
fn given_input() {
    let sequence = keypad_sequence("029A");
    let sequence_out = sequence.iter().copied().map(char::from).collect::<String>();

    assert_eq!(sequence_out, "<A^A^^>AvvvA");

    let dpad_sequence = dpad_sequence(&sequence);
    let dpad_sequence_out = dpad_sequence
        .iter()
        .copied()
        .map(char::from)
        .collect::<String>();

    assert_eq!(
        dpad_sequence_out.len(),
        "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len()
    );

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

    println!(
        "{}\n<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        part1_line("379A")
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
