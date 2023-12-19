use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(usize, usize);

struct Ground {
    lines: Vec<(Point, Point)>,
    digger_position: (usize, usize),
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step_amount(self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl Ground {
    fn new(start_point: (usize, usize), (width, height): (usize, usize)) -> Self {
        Self {
            lines: vec![],
            digger_position: start_point,
        }
    }

    fn dig(&mut self, instruction: Instruction) {
        let step_amount = instruction.0.step_amount();
        let start_point = Point(self.digger_position.0, self.digger_position.1);
        let end_point_x = start_point
            .0
            .checked_add_signed(step_amount.0 * instruction.1)
            .unwrap();
        let end_point_y = start_point
            .1
            .checked_add_signed(step_amount.1 * instruction.1)
            .unwrap();

        self.lines
            .push((start_point, Point(end_point_x, end_point_y)));
    }

    fn total_size(&self) -> usize {
        let mut rectangles = HashSet::new();

        for &(line_start, line_end) in &self.lines {
            // if the line is vertical, don't worry about it
            if line_start.0 == line_end.0 {
                continue;
            }

            // project the start point up and down and find all the lines that intersect them
            let intersection_lines_with_start: HashSet<_> = self
                .lines
                .iter()
                .filter(|test_line| {
                    let x_min = test_line.0 .0.min(test_line.1 .0);
                    let x_max = test_line.0 .0.max(test_line.1 .0);

                    x_min <= line_start.0 && x_max >= line_start.0
                })
                .collect();

            // project the end point up and down and find all the lines that intersect them
            let intersection_lines_with_end: HashSet<_> = self
                .lines
                .iter()
                .filter(|test_line| {
                    let x_min = test_line.0 .0.min(test_line.1 .0);
                    let x_max = test_line.0 .0.max(test_line.1 .0);

                    x_min <= line_end.0 && x_max >= line_end.0
                })
                .collect();

            // cases:
            
        }

        todo!()
    }
}

#[derive(Clone, Copy)]
struct Instruction(Direction, isize);

impl Instruction {
    fn parse(line: &str) -> Self {
        let mut split = line.split(' ');

        let direction = match split.next().unwrap() {
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            c => panic!("Unknown direction {c}"),
        };

        let distance = split.next().unwrap().parse().unwrap();

        Self(direction, distance)
    }

    fn parse_all_input(input: &str) -> Vec<Instruction> {
        input.lines().map(Instruction::parse).collect()
    }
}

fn get_start_point_and_width_height(
    instructions: &[Instruction],
) -> ((usize, usize), (usize, usize)) {
    let mut digger_position = (0isize, 0isize);

    let mut max_x = 0isize;
    let mut max_y = 0isize;

    let mut min_x = 0isize;
    let mut min_y = 0isize;

    for instruction in instructions {
        let distance = instruction.1;
        digger_position = match instruction.0 {
            Direction::Up => (digger_position.0, digger_position.1 - distance),
            Direction::Down => (digger_position.0, digger_position.1 + distance),
            Direction::Left => (digger_position.0 - distance, digger_position.1),
            Direction::Right => (digger_position.0 + distance, digger_position.1),
        };

        max_x = max_x.max(digger_position.0);
        min_x = min_x.min(digger_position.0);

        max_y = max_y.max(digger_position.1);
        min_y = min_y.min(digger_position.1);
    }

    let total_width = max_x.abs_diff(min_x) + 1;
    let total_height = max_y.abs_diff(min_y) + 1;

    let digger_start = (min_x.unsigned_abs(), min_y.unsigned_abs());

    (digger_start, (total_width, total_height))
}

fn part1(input: &str) -> usize {
    let instructions = Instruction::parse_all_input(input);
    let (digger_start, size) = get_start_point_and_width_height(&instructions);

    let mut ground = Ground::new(digger_start, size);

    for instruction in instructions {
        ground.dig(instruction);
    }

    ground.total_size()
}

#[test]
fn given_input() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    assert_eq!(part1(input), 62);
}
