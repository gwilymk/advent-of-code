use std::{collections::VecDeque, fmt::Debug};

use aoc2024::{get_input, Direction, Grid2, Vector2D};

fn main() {
    let input = get_input(15);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
enum Space {
    Wall,
    Box,
    #[default]
    Empty,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default)]
enum Space2 {
    Wall,
    BoxL,
    BoxR,
    #[default]
    Empty,
}

impl Debug for Space2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Space2::Wall => write!(f, "#"),
            Space2::BoxL => write!(f, "["),
            Space2::BoxR => write!(f, "]"),
            Space2::Empty => write!(f, "."),
        }
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

struct Warehouse2 {
    map: Grid2<Space2>,
    robot: Vector2D<i32>,
}

impl Warehouse2 {
    fn parse(input: &str) -> Self {
        let map = Grid2::parse(input, |line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => [Space2::Wall, Space2::Wall],
                    'O' => [Space2::BoxL, Space2::BoxR],
                    '@' | '.' => [Space2::Empty, Space2::Empty],
                    a => panic!("Don't know about an '{a}'"),
                })
                .collect()
        });

        let robot_position = input.chars().position(|c| c == '@').unwrap();
        let robot = Vector2D::new(
            (robot_position % (map.width / 2 + 1)) as i32 * 2,
            (robot_position / (map.width / 2 + 1)) as i32,
        );

        Self { map, robot }
    }

    fn do_moves(&mut self, moves: &[Direction]) {
        'outer: for m in moves {
            let d: Vector2D<_> = m.into();

            let mut map_copy = self.map.clone();

            struct ThingToMove {
                thing_that_is_moving: Space2,
                move_into: Vector2D<i32>,
            }

            impl Debug for ThingToMove {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(
                        f,
                        "{:?} -> ({}, {})",
                        self.thing_that_is_moving, self.move_into.x, self.move_into.y
                    )
                }
            }

            let mut things_to_move = VecDeque::from([ThingToMove {
                thing_that_is_moving: Space2::Empty,
                move_into: self.robot + d,
            }]);

            while let Some(thing_to_move) = things_to_move.pop_front() {
                match map_copy.get::<i32>(thing_to_move.move_into).unwrap() {
                    Space2::Empty => {
                        map_copy.set::<i32>(
                            thing_to_move.move_into,
                            thing_to_move.thing_that_is_moving,
                        );
                    }
                    Space2::Wall => {
                        // we're now stuck
                        continue 'outer;
                    }
                    Space2::BoxL => {
                        map_copy.set::<i32>(
                            thing_to_move.move_into,
                            thing_to_move.thing_that_is_moving,
                        );
                        map_copy.set::<i32>(thing_to_move.move_into + (1, 0).into(), Space2::Empty);

                        match *m {
                            Direction::North | Direction::South | Direction::East => {
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxR,
                                    move_into: thing_to_move.move_into + d + (1, 0).into(),
                                });
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxL,
                                    move_into: thing_to_move.move_into + d,
                                });
                            }
                            Direction::West => {
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxL,
                                    move_into: thing_to_move.move_into + d,
                                });
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxR,
                                    move_into: thing_to_move.move_into + d + (1, 0).into(),
                                });
                            }
                        }
                    }
                    Space2::BoxR => {
                        map_copy.set::<i32>(
                            thing_to_move.move_into,
                            thing_to_move.thing_that_is_moving,
                        );
                        map_copy
                            .set::<i32>(thing_to_move.move_into + (-1, 0).into(), Space2::Empty);

                        match *m {
                            Direction::North | Direction::South | Direction::East => {
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxR,
                                    move_into: thing_to_move.move_into + d,
                                });
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxL,
                                    move_into: thing_to_move.move_into + d + (-1, 0).into(),
                                });
                            }
                            Direction::West => {
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxL,
                                    move_into: thing_to_move.move_into + d + (-1, 0).into(),
                                });
                                things_to_move.push_back(ThingToMove {
                                    thing_that_is_moving: Space2::BoxR,
                                    move_into: thing_to_move.move_into + d,
                                });
                            }
                        }
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
                if thing == Space2::BoxL {
                    Some(pos.x as u32 + pos.y as u32 * 100)
                } else {
                    None
                }
            })
            .sum()
    }
}

impl Debug for Warehouse2 {
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
                            Space2::Wall => '#',
                            Space2::BoxL => '[',
                            Space2::BoxR => ']',
                            Space2::Empty => '.',
                        }
                    )?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn part2(input: &str) -> u32 {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let mut map = Warehouse2::parse(map);
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
    assert_eq!(part2(input), 9021);
}

#[test]
fn given_input3() {
    let input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    assert_eq!(part2(input), 618);
}
