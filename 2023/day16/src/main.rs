fn main() {
    let input = include_str!("../input.txt");
    let contraption = Contraption::parse(input);
    println!("Part 1: {}", part1(&contraption));
    println!("Part 2: {}", part2(&contraption));
}

fn part1(contraption: &Contraption) -> usize {
    count_hits(contraption, (0, 0), LightDirection::Right)
}

fn part2(contraption: &Contraption) -> usize {
    let mut best_so_far = 0;

    for start_x in 0..contraption.width() {
        best_so_far = best_so_far.max(count_hits(contraption, (start_x, 0), LightDirection::Down));
        best_so_far = best_so_far.max(count_hits(
            contraption,
            (start_x, contraption.height() - 1),
            LightDirection::Up,
        ));
    }

    for start_y in 0..contraption.height() {
        best_so_far = best_so_far.max(count_hits(contraption, (0, start_y), LightDirection::Right));
        best_so_far = best_so_far.max(count_hits(
            contraption,
            (contraption.width() - 1, start_y),
            LightDirection::Left,
        ));
    }

    best_so_far
}

fn count_hits(
    contraption: &Contraption,
    start_point: (usize, usize),
    start_direction: LightDirection,
) -> usize {
    let mut result = vec![];
    result.resize_with(contraption.height(), || vec![0; contraption.width()]);

    contraption.simulate(start_point, start_direction, &mut result);

    result
        .iter()
        .flat_map(|row| row.iter().map(|&value| value != 0))
        .map(|hit| if hit { 1 } else { 0 })
        .sum::<usize>()
}

#[derive(Clone, Copy)]
enum Mirror {
    UpLeft,
    UpRight,
    SplitHorizontal,
    SplitVertical,
    Empty,
}

#[derive(Clone, Copy)]
enum LightDirection {
    Up,
    Left,
    Down,
    Right,
}

impl LightDirection {
    fn mask(self) -> u32 {
        1 << self as usize
    }
}

struct Contraption {
    grid: Vec<Vec<Mirror>>,
}

impl Contraption {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|b| match b {
                        b'.' => Mirror::Empty,
                        b'|' => Mirror::SplitHorizontal,
                        b'-' => Mirror::SplitVertical,
                        b'/' => Mirror::UpRight,
                        b'\\' => Mirror::UpLeft,
                        _ => panic!("Unknown character {b}"),
                    })
                    .collect()
            })
            .collect();

        Self { grid }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn simulate(
        &self,
        mut point: (usize, usize),
        mut direction: LightDirection,
        result: &mut [Vec<u32>],
    ) {
        loop {
            let current = result[point.1][point.0];
            if current & direction.mask() != 0 {
                return;
            }
            result[point.1][point.0] = current | direction.mask();

            let mirror = self.grid[point.1][point.0];
            match mirror {
                Mirror::UpLeft => match direction {
                    LightDirection::Up => direction = LightDirection::Left,
                    LightDirection::Left => direction = LightDirection::Up,
                    LightDirection::Down => direction = LightDirection::Right,
                    LightDirection::Right => direction = LightDirection::Down,
                },
                Mirror::UpRight => match direction {
                    LightDirection::Up => direction = LightDirection::Right,
                    LightDirection::Left => direction = LightDirection::Down,
                    LightDirection::Down => direction = LightDirection::Left,
                    LightDirection::Right => direction = LightDirection::Up,
                },
                Mirror::SplitHorizontal => match direction {
                    LightDirection::Up | LightDirection::Down => {}
                    LightDirection::Left | LightDirection::Right => {
                        if let Some(first_simulation_point) =
                            move_point(point, LightDirection::Up, &self.grid)
                        {
                            self.simulate(first_simulation_point, LightDirection::Up, result);
                        };

                        direction = LightDirection::Down;
                    }
                },
                Mirror::SplitVertical => match direction {
                    LightDirection::Left | LightDirection::Right => {}
                    LightDirection::Up | LightDirection::Down => {
                        if let Some(first_simulation_point) =
                            move_point(point, LightDirection::Left, &self.grid)
                        {
                            self.simulate(first_simulation_point, LightDirection::Left, result);
                        };

                        direction = LightDirection::Right;
                    }
                },
                Mirror::Empty => {}
            }

            point = match move_point(point, direction, &self.grid) {
                Some(new_point) => new_point,
                None => return,
            }
        }
    }
}

fn move_point(
    point: (usize, usize),
    direction: LightDirection,
    grid: &[Vec<Mirror>],
) -> Option<(usize, usize)> {
    let new_position = match direction {
        LightDirection::Up => (point.0, point.1.checked_sub(1)?),
        LightDirection::Left => (point.0.checked_sub(1)?, point.1),
        LightDirection::Down => (point.0, point.1 + 1),
        LightDirection::Right => (point.0 + 1, point.1),
    };

    if new_position.1 == grid.len() {
        return None;
    }

    if new_position.0 == grid[0].len() {
        return None;
    }

    Some(new_position)
}

#[test]
fn test_given_input_simulate() {
    let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    assert_eq!(part1(input), 46);
}
