fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", area(input, InputParseStyle::Part1));
    println!("Part 2: {}", area(input, InputParseStyle::Part2));
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

struct Ground {
    lines: Vec<Point>,
    digger_position: (isize, isize),
}

#[derive(Debug, Clone, Copy)]
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
    fn new() -> Self {
        Self {
            lines: vec![Point(0, 0)],
            digger_position: (0, 0),
        }
    }

    fn dig(&mut self, instruction: Instruction) {
        let step_amount = instruction.0.step_amount();
        let start_point = Point(self.digger_position.0, self.digger_position.1);
        let end_point_x = start_point.0 + step_amount.0 * instruction.1;
        let end_point_y = start_point.1 + step_amount.1 * instruction.1;

        self.digger_position = (end_point_x, end_point_y);

        self.lines.push(Point(end_point_x, end_point_y));
    }

    fn total_size(&self) -> usize {
        // similar to the shoelace theorem, except that everything is axis aligned
        // so we don't have to worry about differing y values.
        //
        // Calculate the area and then use a rearranged pick's theorem to calculate
        // the required size

        let mut points = self.lines.clone();
        points.push(self.lines[0]);

        let area = points
            .windows(2)
            .map(|window| {
                let Point(x1, y1) = window[0];
                let Point(x2, _) = window[1];

                (x1 - x2) * y1
            })
            .sum::<isize>()
            .unsigned_abs();

        let perimeter = points
            .windows(2)
            .map(|window| {
                let Point(x1, y1) = window[0];
                let Point(x2, y2) = window[1];

                (x1 - x2).unsigned_abs() + (y1 - y2).unsigned_abs()
            })
            .sum::<usize>();

        area + perimeter / 2 + 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction(Direction, isize);

impl Instruction {
    fn parse(line: &str, input_parse_style: InputParseStyle) -> Self {
        let mut split = line.split(' ');

        match input_parse_style {
            InputParseStyle::Part1 => {
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
            InputParseStyle::Part2 => {
                let instruction = split.skip(2).next().unwrap();

                let distance = isize::from_str_radix(&instruction[2..7], 16).unwrap();
                let direction = match instruction.chars().nth(7).unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    c => panic!("Unknown direction {c} in instruction {instruction}"),
                };

                Self(direction, distance)
            }
        }
    }

    fn parse_all_input(input: &str, input_parse_style: InputParseStyle) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| Instruction::parse(line, input_parse_style))
            .collect()
    }
}

#[derive(Clone, Copy)]
enum InputParseStyle {
    Part1,
    Part2,
}

fn area(input: &str, input_parsing_style: InputParseStyle) -> usize {
    let instructions = Instruction::parse_all_input(input, input_parsing_style);

    let mut ground = Ground::new();

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

    assert_eq!(area(input, InputParseStyle::Part1), 62);
    assert_eq!(area(input, InputParseStyle::Part2), 952408144115);
}
