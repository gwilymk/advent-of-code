use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let grid = Grid::parse(input, Connection::NorthSouth);
    println!("Part 1: {}", grid.loop_length() / 2);
    println!("Part 2: {}", enclosed_area(&grid));
}

struct Grid {
    starting_position: (usize, usize),
    paths: Vec<Vec<Connection>>,
}

impl Grid {
    fn parse(input: &str, animal_start_connection: Connection) -> Self {
        let mut paths: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.bytes().map(Connection::parse).collect())
            .collect();

        let mut animal_start = (0, 0);

        for (y, row) in paths.iter_mut().enumerate() {
            for (x, connection) in row.iter_mut().enumerate() {
                if *connection == Connection::Animal {
                    animal_start = (x, y);
                    *connection = animal_start_connection;
                }
            }
        }

        Self {
            paths,
            starting_position: animal_start,
        }
    }

    fn get_loop(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];
        let mut position = self.starting_position;
        let mut previous_position = position;

        loop {
            result.push(position);
            let potential_directions = self.paths[position.1][position.0].next_positions(position);

            if potential_directions.0 == previous_position {
                previous_position = position;
                position = potential_directions.1;
            } else {
                previous_position = position;
                position = potential_directions.0;
            }

            if position == self.starting_position {
                break result;
            }
        }
    }

    fn loop_length(&self) -> usize {
        self.get_loop().len()
    }

    fn non_loop(&self) -> Vec<(usize, usize)> {
        let loop_ = self.get_loop();
        let loop_: HashSet<_> = loop_.iter().collect();

        let height = self.paths.len();
        let width = self.paths[0].len();

        let mut result = vec![];

        for y in 0..height {
            for x in 0..width {
                if !loop_.contains(&(x, y)) {
                    result.push((x, y));
                }
            }
        }

        result
    }
}

fn does_enclose(loop_: &[(usize, usize)], (x, y): (usize, usize)) -> bool {
    let mut hit_directions = vec![];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    for &(loop_x, loop_y) in loop_ {
        if loop_x == x {
            if loop_y > y {
                hit_directions.push(Direction::Down);
            } else {
                hit_directions.push(Direction::Up);
            }
        }

        if loop_y == y {
            if loop_x > x {
                hit_directions.push(Direction::Right);
            } else {
                hit_directions.push(Direction::Left);
            }
        }
    }

    let mut winding_number = 0;

    if hit_directions.is_empty() {
        return false;
    }

    hit_directions.push(hit_directions[0]);
    let mut previous_direction = hit_directions[0];

    for &hit_direction in &hit_directions {
        if hit_direction == previous_direction {
            continue;
        }

        winding_number += match (previous_direction, hit_direction) {
            (Direction::Up, Direction::Left) => -1,
            (Direction::Up, Direction::Right) => 1,
            (Direction::Down, Direction::Left) => 1,
            (Direction::Down, Direction::Right) => -1,
            (Direction::Left, Direction::Up) => 1,
            (Direction::Left, Direction::Down) => -1,
            (Direction::Right, Direction::Up) => -1,
            (Direction::Right, Direction::Down) => 1,
            _ => panic!("Invalid directions {previous_direction:?} {hit_direction:?}"),
        };

        previous_direction = hit_direction;
    }

    winding_number % 4 == 0 && winding_number != 0
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Connection {
    NorthSouth,
    NorthEast,
    NorthWest,
    EastWest,
    EastSouth,
    SouthWest,

    Ground,
    Animal,
}

impl Connection {
    /*
        | is a vertical pipe connecting north and south.
        - is a horizontal pipe connecting east and west.
        L is a 90-degree bend connecting north and east.
        J is a 90-degree bend connecting north and west.
        7 is a 90-degree bend connecting south and west.
        F is a 90-degree bend connecting south and east.
        . is ground; there is no pipe in this tile.
        S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    */
    fn parse(input: u8) -> Connection {
        match input {
            b'|' => Connection::NorthSouth,
            b'-' => Connection::EastWest,
            b'L' => Connection::NorthEast,
            b'J' => Connection::NorthWest,
            b'7' => Connection::SouthWest,
            b'F' => Connection::EastSouth,
            b'.' => Connection::Ground,
            b'S' => Connection::Animal,
            _ => panic!("Unknown character {input}"),
        }
    }

    fn next_positions(self, (x, y): (usize, usize)) -> ((usize, usize), (usize, usize)) {
        match self {
            Connection::NorthSouth => ((x, y + 1), (x, y - 1)),
            Connection::NorthEast => ((x, y - 1), (x + 1, y)),
            Connection::NorthWest => ((x, y - 1), (x - 1, y)),
            Connection::EastWest => ((x - 1, y), (x + 1, y)),
            Connection::EastSouth => ((x + 1, y), (x, y + 1)),
            Connection::SouthWest => ((x - 1, y), (x, y + 1)),
            Connection::Ground | Connection::Animal => panic!("Bad type {self:?}"),
        }
    }
}

fn enclosed_area(grid: &Grid) -> usize {
    let enclosed_loop = grid.get_loop();
    grid.non_loop()
        .iter()
        .filter(|&&pos| does_enclose(&enclosed_loop, pos))
        .count()
}

#[test]
fn given_input() {
    let text = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    let grid = Grid::parse(text, Connection::EastSouth);
    assert_eq!(grid.loop_length(), 16);
}

#[test]
fn part2_given_input() {
    let text = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    let grid = Grid::parse(text, Connection::EastSouth);
    assert_eq!(enclosed_area(&grid), 4);
}

#[test]
fn part2_given_input2() {
    let text = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    let grid = Grid::parse(text, Connection::EastSouth);
    assert_eq!(enclosed_area(&grid), 8);
}

#[test]
fn part2_given_input3() {
    let text = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    let grid = Grid::parse(text, Connection::SouthWest);
    assert_eq!(enclosed_area(&grid), 10);
}
