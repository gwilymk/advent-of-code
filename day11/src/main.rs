use std::fmt::{self, Debug, Formatter};

fn main() {
    let input = include_str!("../input.txt");
    let mut star_map = StarMap::parse(input);
    star_map.expand();

    println!("Part 1: {}", star_map.sum_of_distances());
}

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
                        b'.' => StarMapEntry::Empty,
                        b'#' => StarMapEntry::Galaxy,
                        _ => panic!("Unknown character {byte}"),
                    })
                    .collect()
            })
            .collect();

        Self { content }
    }

    pub fn expand(&mut self) {
        let empty_columns: Vec<_> = (0..self.content[0].len())
            .filter(|&column| {
                self.content
                    .iter()
                    .all(|row| matches!(row[column], StarMapEntry::Empty))
            })
            .collect();

        for &empty_column in empty_columns.iter().rev() {
            for row in &mut self.content {
                row.insert(empty_column, StarMapEntry::Empty);
            }
        }

        let empty_rows: Vec<_> = (0..self.content.len())
            .filter(|&row| {
                self.content[row]
                    .iter()
                    .all(|entry| matches!(entry, StarMapEntry::Empty))
            })
            .collect();

        let width = self.content[0].len();
        for &empty_row in empty_rows.iter().rev() {
            self.content
                .insert(empty_row, vec![StarMapEntry::Empty; width]);
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
                let distance = galaxy_position2.0.abs_diff(galaxy_position1.0)
                    + galaxy_position2.1.abs_diff(galaxy_position1.1);
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
                    StarMapEntry::Empty => write!(f, ".")?,
                    StarMapEntry::Galaxy => write!(f, "#")?,
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
enum StarMapEntry {
    #[default]
    Empty,
    Galaxy,
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
    star_map.expand();

    assert_eq!(star_map.sum_of_distances(), 374);
}
