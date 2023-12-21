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
    start_point: (usize, usize),
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
                        start_point = (x, y);
                        Tile::GardenPlot
                    }
                    _ => panic!("Unknown character {c}"),
                };

                map_line.push(tile);
            }

            map.push(map_line);
        }

        Self { start_point, map }
    }

    fn get_point(&self, pos: (usize, usize)) -> Option<Tile> {
        self.map
            .get(pos.1)
            .and_then(|line| line.get(pos.0))
            .copied()
    }

    fn get_point_offset(&self, pos: (usize, usize), offset: (isize, isize)) -> Option<Tile> {
        self.get_point((
            pos.0.checked_add_signed(offset.0)?,
            pos.1.checked_add_signed(offset.1)?,
        ))
    }

    pub fn possible_positions(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];

        if self.get_point_offset(pos, (1, 0)) == Some(Tile::GardenPlot) {
            res.push((pos.0 + 1, pos.1));
        }

        if self.get_point_offset(pos, (-1, 0)) == Some(Tile::GardenPlot) {
            res.push((pos.0 - 1, pos.1));
        }

        if self.get_point_offset(pos, (0, 1)) == Some(Tile::GardenPlot) {
            res.push((pos.0, pos.1 + 1));
        }

        if self.get_point_offset(pos, (0, -1)) == Some(Tile::GardenPlot) {
            res.push((pos.0, pos.1 - 1));
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
            for possible_position in map.possible_positions(position) {
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
