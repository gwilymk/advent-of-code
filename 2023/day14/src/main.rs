use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("../input.txt");
    let mut ground = Ground::parse(input);
    let spin_ground = ground.clone();
    ground.tilt_north();

    println!("Part 1: {}", ground.load_on_north_beam());
    println!("Part 2: {}", spin_cycle(&spin_ground, 1000000000));
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Ground {
    ground: Vec<Vec<Content>>,
}

impl Ground {
    fn parse(input: &str) -> Self {
        let ground = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|c| match c {
                        b'.' => Content::Empty,
                        b'O' => Content::RoundRock,
                        b'#' => Content::SquareRock,
                        _ => panic!("Unknown character {c}"),
                    })
                    .collect()
            })
            .collect();

        Self { ground }
    }

    fn tilt_north(&mut self) {
        for x in 0..self.ground[0].len() {
            for y_start in 0..self.ground.len() {
                if self.ground[y_start][x] == Content::Empty {
                    for y in y_start..self.ground.len() {
                        match self.ground[y][x] {
                            Content::Empty => continue,
                            Content::RoundRock => {
                                self.ground[y_start][x] = Content::RoundRock;
                                self.ground[y][x] = Content::Empty;
                                break;
                            }
                            Content::SquareRock => break,
                        }
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.ground[0].len() {
            for y_start in (0..self.ground.len()).rev() {
                if self.ground[y_start][x] == Content::Empty {
                    for y in (0..y_start).rev() {
                        match self.ground[y][x] {
                            Content::Empty => continue,
                            Content::RoundRock => {
                                self.ground[y_start][x] = Content::RoundRock;
                                self.ground[y][x] = Content::Empty;
                                break;
                            }
                            Content::SquareRock => break,
                        }
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in &mut self.ground {
            for x_start in 0..row.len() {
                if row[x_start] == Content::Empty {
                    for x in x_start..row.len() {
                        match row[x] {
                            Content::Empty => continue,
                            Content::RoundRock => {
                                row[x_start] = Content::RoundRock;
                                row[x] = Content::Empty;
                                break;
                            }
                            Content::SquareRock => break,
                        }
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in &mut self.ground {
            for x_start in (0..row.len()).rev() {
                if row[x_start] == Content::Empty {
                    for x in (0..x_start).rev() {
                        match row[x] {
                            Content::Empty => continue,
                            Content::RoundRock => {
                                row[x_start] = Content::RoundRock;
                                row[x] = Content::Empty;
                                break;
                            }
                            Content::SquareRock => break,
                        }
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn load_on_north_beam(&self) -> usize {
        let height = self.ground.len();

        self.ground
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let multiplier = height - y;
                row.iter()
                    .filter(|&&value| value == Content::RoundRock)
                    .count()
                    * multiplier
            })
            .sum::<usize>()
    }
}

fn spin_cycle(ground: &Ground, loops: usize) -> usize {
    let mut cache = HashMap::new();

    let mut ground_to_spin = ground.clone();

    let mut num_loops = 0usize;

    let loop_length = loop {
        num_loops += 1;

        ground_to_spin.cycle();
        let cached = cache.insert(ground_to_spin.clone(), num_loops);

        if let Some(cached) = cached {
            break num_loops - cached;
        }
    };

    let bit_before_looping = num_loops - loop_length;

    let cache_value_to_use = (loops - bit_before_looping) % loop_length + bit_before_looping;

    cache
        .iter()
        .find_map(|(ground, &value)| {
            if cache_value_to_use == value {
                Some(ground.load_on_north_beam())
            } else {
                None
            }
        })
        .unwrap()
}

impl Debug for Ground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.ground {
            for c in row {
                match c {
                    Content::Empty => write!(f, ".")?,
                    Content::RoundRock => write!(f, "#")?,
                    Content::SquareRock => write!(f, "O")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Content {
    Empty,
    RoundRock,
    SquareRock,
}

#[test]
fn load_on_north_beam_given_input() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    let mut ground = Ground::parse(input);
    ground.tilt_north();
    assert_eq!(ground.load_on_north_beam(), 136);
}

#[test]
fn tilt_round_cycle() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    let mut ground = Ground::parse(input);
    ground.tilt_north();

    ground.cycle();

    let cycle1 = Ground::parse(
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
    );

    assert_eq!(ground, cycle1);
}
