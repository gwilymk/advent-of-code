use std::collections::HashSet;

fn main() {
    let map = GardenMap::parse(include_str!("../input.txt"));
    println!("Part 1: {}", part1(&map, 64));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock,
}

struct GardenMap {
    map: Vec<Vec<Tile>>,
    start_point: (isize, isize),

    width: usize,
    height: usize,
}

impl GardenMap {
    pub fn parse(input: &str) -> Self {
        let mut map = vec![];
        let mut start_point = (0, 0);

        for (y, line) in input.lines().enumerate() {
            let mut map_line = vec![];
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::GardenPlot,
                    '#' => Tile::Rock,
                    'S' => {
                        start_point = (x as _, y as _);
                        Tile::GardenPlot
                    }
                    _ => panic!("Unknown character {c}"),
                };

                map_line.push(tile);
            }

            map.push(map_line);
        }

        let width = map[0].len();
        let height = map.len();

        Self {
            start_point,
            map,
            width,
            height,
        }
    }

    fn get_point(&self, (x, y): (isize, isize), is_infinite: bool) -> Option<Tile> {
        let x = if is_infinite {
            x.rem_euclid(self.width as isize) as usize
        } else {
            if x < 0 {
                return None;
            }

            x as usize
        };

        let y = if is_infinite {
            y.rem_euclid(self.height as isize) as usize
        } else {
            if y < 0 {
                return None;
            }

            y as usize
        };

        self.map.get(y).and_then(|line| line.get(x)).copied()
    }

    fn calculate_point_offset(
        &self,
        pos: (isize, isize),
        offset: (isize, isize),
    ) -> (isize, isize) {
        (pos.0 + offset.0, pos.1 + offset.1)
    }

    pub fn possible_positions(
        &self,
        pos: (isize, isize),
        is_infinite: bool,
    ) -> Vec<(isize, isize)> {
        let mut res = vec![];

        let up = self.calculate_point_offset(pos, (0, -1));
        let down = self.calculate_point_offset(pos, (0, 1));
        let left = self.calculate_point_offset(pos, (-1, 0));
        let right = self.calculate_point_offset(pos, (1, 0));

        if self.get_point(up, is_infinite) == Some(Tile::GardenPlot) {
            res.push(up);
        }

        if self.get_point(down, is_infinite) == Some(Tile::GardenPlot) {
            res.push(down);
        }

        if self.get_point(left, is_infinite) == Some(Tile::GardenPlot) {
            res.push(left);
        }

        if self.get_point(right, is_infinite) == Some(Tile::GardenPlot) {
            res.push(right);
        }

        res
    }
}

fn part1(map: &GardenMap, steps: usize) -> usize {
    let mut positions = HashSet::new();
    positions.insert(map.start_point);

    for _ in 0..steps {
        let mut new_positions = HashSet::new();

        for position in positions {
            for possible_position in map.possible_positions(position, false) {
                new_positions.insert(possible_position);
            }
        }

        positions = new_positions;
    }

    positions.len()
}

fn part2(map: &GardenMap, steps: usize) -> usize {
    let mut positions = HashSet::new();
    positions.insert(map.start_point);

    for _ in 0..steps {
        let mut new_positions = HashSet::new();

        for position in positions {
            for possible_position in map.possible_positions(position, true) {
                new_positions.insert(possible_position);
            }
        }

        positions = new_positions;
    }

    positions.len()
}

#[test]
fn given_input_part1() {
    let map = GardenMap::parse(
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    );

    assert_eq!(part1(&map, 6), 16);
}

#[test]
fn given_input_part2() {
    let map = GardenMap::parse(
        "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........",
    );

    assert_eq!(part2(&map, 6), 16);
    assert_eq!(part2(&map, 10), 50);
    assert_eq!(part2(&map, 50), 1594);
    assert_eq!(part2(&map, 500), 167004);
}
