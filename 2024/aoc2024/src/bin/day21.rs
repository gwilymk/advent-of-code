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

fn keypad_from_coordinate(coord: Vector2D<i32>) -> Option<char> {
    Some(match (coord.x, coord.y) {
        (0, 0) => '7',
        (1, 0) => '8',
        (2, 0) => '9',
        (0, 1) => '4',
        (1, 1) => '5',
        (2, 1) => '6',
        (0, 2) => '1',
        (1, 2) => '2',
        (2, 2) => '3',
        (1, 3) => '0',
        (2, 3) => 'A',
        _ => return None,
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

fn dpad_from_coordinate(coord: Vector2D<i32>) -> DpadInstruction {}

impl DpadInstruction {
    fn all() -> [DpadInstruction; 5] {
        [
            DpadInstruction::A,
            DpadInstruction::Direction(Direction::North),
            DpadInstruction::Direction(Direction::East),
            DpadInstruction::Direction(Direction::South),
            DpadInstruction::Direction(Direction::West),
        ]
    }

    fn apply(self, instruction: &mut DpadInstruction) -> Result<Option<DpadInstruction>, ()> {
        match self {
            DpadInstruction::A => Ok(Some(*instruction)),
            DpadInstruction::Direction(direction) => {
                *instruction = instruction.move_direction(direction).ok_or(())?;
                Ok(None)
            }
        }
    }

    fn move_direction(self, direction: Direction) -> Option<Self> {
        Some(match direction {
            Direction::North => match self {
                DpadInstruction::Direction(Direction::South) => {
                    DpadInstruction::Direction(Direction::North)
                }
                DpadInstruction::Direction(Direction::East) => DpadInstruction::A,
                _ => return None,
            },
            Direction::East => match self {
                DpadInstruction::A => DpadInstruction::Direction(Direction::North),
                DpadInstruction::Direction(Direction::East) => {
                    DpadInstruction::Direction(Direction::South)
                }
                DpadInstruction::Direction(Direction::South) => {
                    DpadInstruction::Direction(Direction::West)
                }
                _ => return None,
            },
            Direction::South => match self {
                DpadInstruction::A => DpadInstruction::Direction(Direction::East),
                DpadInstruction::Direction(Direction::North) => {
                    DpadInstruction::Direction(Direction::South)
                }
                _ => return None,
            },
            Direction::West => match self {
                DpadInstruction::Direction(Direction::West) => {
                    DpadInstruction::Direction(Direction::South)
                }
                DpadInstruction::Direction(Direction::South) => {
                    DpadInstruction::Direction(Direction::East)
                }
                DpadInstruction::Direction(Direction::North) => DpadInstruction::A,
                _ => return None,
            },
        })
    }
}

fn part1_line(input: &str) -> String {
    #[derive(Clone, PartialEq, Eq, Hash)]
    struct State {
        digits: usize,

        keypad_arm: char,
        dpad1_arm: DpadInstruction,
        dpad2_arm: DpadInstruction,
    }

    impl State {
        fn neighbours(&self) -> Vec<State> {
            let mut neighbours = vec![];

            // for
        }
    }
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
