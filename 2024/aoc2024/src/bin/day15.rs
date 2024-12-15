use std::fmt::Debug;

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(15);
    println!("Part 1: {}", part1(&input));
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
enum Space {
    Wall,
    Box,
    #[default]
    Empty,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn parse(c: char) -> Option<Self> {
        Some(match c {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => return None,
        })
    }
}

impl From<&Direction> for Vector2D<i32> {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
        .into()
    }
}

struct Warehouse {
    map: Grid2<Space>,
    robot: Vector2D<i32>,
}

impl Warehouse {
    fn parse(input: &str) -> Self {
        let map = Grid2::parse(input, |line| {
            line.chars()
                .map(|c| match c {
                    '#' => Space::Wall,
                    'O' => Space::Box,
                    '@' | '.' => Space::Empty,
                    a => panic!("Don't know about an '{a}'"),
                })
                .collect()
        });

        let robot_position = input.chars().position(|c| c == '@').unwrap();
        let robot = Vector2D::new(
            (robot_position % (map.width + 1)) as i32,
            (robot_position / (map.width + 1)) as i32,
        );

        Self { map, robot }
    }

    fn do_moves(&mut self, moves: &[Direction]) {
        'outer: for m in moves {
            let d: Vector2D<_> = m.into();

            let mut map_copy = self.map.clone();
            let mut thing_to_move_pos = self.robot + d;
            let mut thing_that_is_moving = Space::Empty;
            loop {
                match map_copy.get::<i32>(thing_to_move_pos).unwrap() {
                    Space::Empty => {
                        map_copy.set::<i32>(thing_to_move_pos, thing_that_is_moving);
                        break;
                    }
                    Space::Wall => {
                        // we're now stuck
                        continue 'outer;
                    }
                    Space::Box => {
                        map_copy.set::<i32>(thing_to_move_pos, thing_that_is_moving);
                        thing_that_is_moving = Space::Box;
                        thing_to_move_pos += d;
                    }
                }
            }

            self.map = map_copy;
            self.robot += d;
        }
    }

    fn gps(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|(pos, &thing)| {
                if thing == Space::Box {
                    Some(pos.x as u32 + pos.y as u32 * 100)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl Debug for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.height as i32 {
            for x in 0..self.map.width as i32 {
                let space = *self.map.get::<i32>((x, y)).unwrap();

                if self.robot == (x, y).into() {
                    write!(f, "@")?;
                } else {
                    write!(
                        f,
                        "{}",
                        match space {
                            Space::Wall => '#',
                            Space::Box => 'O',
                            Space::Empty => '.',
                        }
                    )?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(input: &str) -> u32 {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let mut map = Warehouse::parse(map);
    let directions = directions
        .chars()
        .filter_map(Direction::parse)
        .collect::<Vec<_>>();

    map.do_moves(&directions);
    map.gps()
}

#[test]
fn given_input1() {
    let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    assert_eq!(part1(input), 2028);
}

#[test]
fn given_input2() {
    let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    assert_eq!(part1(input), 10092);
}
