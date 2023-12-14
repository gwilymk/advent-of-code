fn main() {
    let input = include_str!("../input.txt");
    let mut ground = Ground::parse(input);
    ground.tilt_north();

    println!("Part 1: {}", ground.load_on_north_beam());
}

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

#[derive(Clone, Copy, PartialEq, Eq)]
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
