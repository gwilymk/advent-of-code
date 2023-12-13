use std::fmt::Debug;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", solve(input, 0));
    println!("Part 2: {}", solve(input, 1));
}

fn solve(input: &str, smudges: usize) -> usize {
    input
        .split("\n\n")
        .map(Grid::parse)
        .map(|grid| grid.value(smudges))
        .sum::<usize>()
}

struct Grid {
    ground: Vec<Vec<Terrain>>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let ground = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|c| match c {
                        b'.' => Terrain::Ash,
                        b'#' => Terrain::Rock,
                        _ => panic!("Invalid character {c}"),
                    })
                    .collect()
            })
            .collect();
        Self { ground }
    }

    fn reflection_column(&self, smudges: usize) -> Option<usize> {
        (1..self.ground[0].len()).find(|&x| {
            self.ground
                .iter()
                .map(|row| {
                    let amount = x.min(row.len() - x);
                    let before_line = row.iter().rev().skip(row.len() - x).take(amount);
                    let after_line = row.iter().skip(x).take(amount);

                    before_line.zip(after_line).filter(|(a, b)| a != b).count()
                })
                .sum::<usize>()
                == smudges
        })
    }

    fn reflection_row(&self, smudges: usize) -> Option<usize> {
        (1..self.ground.len()).find(|&y| {
            (0..self.ground[0].len())
                .map(|x| {
                    let column: Vec<_> = self.ground.iter().map(|row| row[x]).collect();

                    let amount = y.min(column.len() - y);
                    let before_line = column.iter().rev().skip(column.len() - y).take(amount);
                    let after_line = column.iter().skip(y).take(amount);

                    before_line.zip(after_line).filter(|(a, b)| a != b).count()
                })
                .sum::<usize>()
                == smudges
        })
    }

    fn value(&self, smudges: usize) -> usize {
        self.reflection_column(smudges).unwrap_or(0)
            + self.reflection_row(smudges).unwrap_or(0) * 100
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.ground {
            for terrain in row {
                match terrain {
                    Terrain::Ash => write!(f, ".")?,
                    Terrain::Rock => write!(f, "#")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Ash,
    Rock,
}

#[test]
fn can_find_column_of_reflection() {
    let grid = Grid::parse(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
    );

    assert_eq!(grid.reflection_column(0), Some(5));
    assert_eq!(grid.reflection_row(0), None)
}

#[test]
fn can_find_row_of_reflection() {
    let grid = Grid::parse(
        "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    );

    assert_eq!(grid.reflection_row(0), Some(4));
    assert_eq!(grid.reflection_column(0), None);
}

#[test]
fn can_find_at_edge() {
    let grid = Grid::parse(
        "#####.##.####.###
...#.####...#....
##.###.#..##.####
.##..##...##..#..
.######.....#####
##.#..######.####
.##.###.##..##...
...#####..#.#...#
...##..#.....#.##
..#.#...#......##
..#.#...#......##",
    );

    assert_eq!(grid.value(0), 1000);
}
