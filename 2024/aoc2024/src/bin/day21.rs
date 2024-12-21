use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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

impl DpadInstruction {
    fn from_coord(coord: Vector2D<i32>) -> Option<Self> {
        Some(match (coord.x, coord.y) {
            (2, 0) => DpadInstruction::A,
            (1, 0) => DpadInstruction::Direction(Direction::North),
            (2, 1) => DpadInstruction::Direction(Direction::East),
            (1, 1) => DpadInstruction::Direction(Direction::South),
            (0, 1) => DpadInstruction::Direction(Direction::West),
            _ => return None,
        })
    }

    fn to_coord(self) -> Vector2D<i32> {
        match self {
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
        let coord = self.to_coord();
        let new_coord = coord + direction.into();
        Self::from_coord(new_coord)
    }
}

fn part1_line(input: &str) -> usize {
    #[derive(Clone, PartialEq, Eq, Debug)]
    struct Node {
        distance: usize,
        state: State,
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.distance.cmp(&other.distance)
        }
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut q = BinaryHeap::new();
    let mut distance: HashMap<State, usize> = HashMap::new();

    q.push(Reverse(Node {
        distance: 0,
        state: State {
            digits: 0,
            keypad_arm: 'A',
            dpad1_arm: DpadInstruction::A,
            dpad2_arm: DpadInstruction::A,
        },
    }));

    let input = input.chars().collect::<Vec<_>>();

    while let Some(Reverse(minimum)) = q.pop() {
        if minimum.state.digits == input.len() {
            return minimum.distance;
        }

        for neighbour in minimum.state.neighbours(&input) {
            let neighbour_distance = minimum.distance + 1;
            let current_distance = *distance.get(&neighbour).unwrap_or(&usize::MAX);

            if neighbour_distance < current_distance {
                distance.insert(neighbour.clone(), neighbour_distance);
                q.push(Reverse(Node {
                    distance: neighbour_distance,
                    state: neighbour,
                }));
            }
        }
    }

    panic!("No way to do this :(");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    digits: usize,

    keypad_arm: char,
    dpad1_arm: DpadInstruction,
    dpad2_arm: DpadInstruction,
}

impl State {
    fn neighbours(&self, input: &[char]) -> Vec<State> {
        let mut neighbours = vec![];

        for instr in DpadInstruction::all() {
            let mut working = self.clone();

            match instr.apply(&mut working.dpad2_arm) {
                Err(_) => continue,
                Ok(None) => {}
                Ok(Some(instr2)) => {
                    match instr2.apply(&mut working.dpad1_arm) {
                        Err(_) => continue,
                        Ok(None) => {}
                        Ok(Some(instr2)) => {
                            match instr2 {
                                DpadInstruction::A => {
                                    // dpad2 is having the A button pressed, so extend the digits
                                    let current_digit = working.keypad_arm;
                                    if current_digit == input[working.digits] {
                                        working.digits += 1;
                                    } else {
                                        continue; // invalid
                                    }
                                }
                                DpadInstruction::Direction(direction) => {
                                    let keypad_place = keypad_coordinate(working.keypad_arm);
                                    if let Some(new_place) =
                                        keypad_from_coordinate(keypad_place + direction.into())
                                    {
                                        working.keypad_arm = new_place;
                                    } else {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            neighbours.push(working);
        }

        neighbours
    }
}

fn part1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line: &str| {
            let dpad2 = part1_line(line);

            let number = line[..line.len() - 1].parse::<usize>().unwrap();

            number * dpad2
        })
        .sum()
}

#[test]
fn given_input() {
    assert_eq!(
        part1_line("029A"),
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("980A"),
        "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len()
    );

    assert_eq!(
        part1_line("179A"),
        "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("456A"),
        "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len()
    );

    assert_eq!(
        part1_line("379A"),
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
