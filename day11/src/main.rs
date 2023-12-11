use std::fmt::{self, Debug, Formatter};

fn main() {
    let input = include_str!("../input.txt");
    let mut star_map = StarMap::parse(input);
    let mut star_map2 = star_map.clone();

    star_map.expand(2);

    println!("Part 1: {}", star_map.sum_of_distances());

    star_map2.expand(1_000_000);
    println!("Part 2: {}", star_map2.sum_of_distances());
}

#[derive(Clone)]
struct StarMap {
    content: Vec<Vec<StarMapEntry>>,
}

impl StarMap {
    pub fn parse(input: &str) -> Self {
        let content: Vec<Vec<StarMapEntry>> = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|byte| match byte {
                        b'.' => StarMapEntry::Empty(1),
                        b'#' => StarMapEntry::Galaxy,
                        _ => panic!("Unknown character {byte}"),
                    })
                    .collect()
            })
            .collect();

        Self { content }
    }

    pub fn expand(&mut self, amount: usize) {
        let empty_columns: Vec<_> = (0..self.content[0].len())
            .filter(|&column| {
                self.content
                    .iter()
                    .all(|row| matches!(row[column], StarMapEntry::Empty(_)))
            })
            .collect();

        for empty_column in empty_columns {
            for row in &mut self.content {
                row[empty_column] = StarMapEntry::Empty(amount);
            }
        }

        let empty_rows: Vec<_> = (0..self.content.len())
            .filter(|&row| {
                self.content[row]
                    .iter()
                    .all(|entry| matches!(entry, StarMapEntry::Empty(_)))
            })
            .collect();

        for empty_row in empty_rows {
            for item in &mut self.content[empty_row] {
                *item = StarMapEntry::Empty(amount);
            }
        }
    }

    fn sum_of_distances(&self) -> usize {
        let mut galaxy_positions = vec![];

        for (y, row) in self.content.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                if matches!(entry, StarMapEntry::Galaxy) {
                    galaxy_positions.push((x, y));
                }
            }
        }

        let mut total = 0;
        for (index, galaxy_position1) in galaxy_positions.iter().enumerate() {
            for galaxy_position2 in galaxy_positions.iter().skip(index + 1) {
                let x_min = galaxy_position1.0.min(galaxy_position2.0);
                let x_max = galaxy_position1.0.max(galaxy_position2.0);

                let y_min = galaxy_position1.1.min(galaxy_position2.1);
                let y_max = galaxy_position1.1.max(galaxy_position2.1);

                let y_distance = self.content[y_min..y_max]
                    .iter()
                    .map(|item| item[x_min].distance())
                    .sum::<usize>();

                let x_distance = self.content[y_max][x_min..x_max]
                    .iter()
                    .map(|item| item.distance())
                    .sum::<usize>();

                let distance = x_distance + y_distance;
                total += distance;
            }
        }

        total
    }
}

impl Debug for StarMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.content {
            for entry in row {
                match entry {
                    StarMapEntry::Empty(distance) => write!(f, "{distance}")?,
                    StarMapEntry::Galaxy => write!(f, "#")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum StarMapEntry {
    Empty(usize),
    Galaxy,
}

impl StarMapEntry {
    pub fn distance(&self) -> usize {
        match self {
            StarMapEntry::Empty(space) => *space,
            StarMapEntry::Galaxy => 1,
        }
    }
}

#[test]
fn given_case() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let mut star_map = StarMap::parse(input);
    star_map.expand(2);

    assert_eq!(star_map.sum_of_distances(), 374);
}

#[test]
fn given_case_part2() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    let mut star_map: StarMap = StarMap::parse(input);
    star_map.expand(10);

    assert_eq!(star_map.sum_of_distances(), 1030);
}
