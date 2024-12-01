use std::{fs, ops::Range, str};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let schematic = Schematic::parse(&input);

    println!("Part 1: {}", schematic.sum_of_parts());
    println!("Part 2: {}", schematic.sum_of_gear_ratios());
}

struct Schematic {
    items: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct PartLocation {
    y: usize,
    x: Range<usize>,
    number: i32,
}

impl Schematic {
    pub fn parse(input: &str) -> Schematic {
        let items = input.lines().map(|line| line.as_bytes().to_vec()).collect();
        Schematic { items }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.items
            .get(y)
            .and_then(|row: &Vec<u8>| row.get(x))
            .copied()
    }

    fn is_adjacent_to_symbol(&self, x_start: usize, x_end: usize, y: usize) -> bool {
        for y in y.saturating_sub(1)..=(y + 1) {
            for x in x_start.saturating_sub(1)..(x_end + 1) {
                let Some(c) = self.get(x, y) else {
                    continue;
                };

                if !c.is_ascii_digit() && c != b'.' {
                    return true;
                }
            }
        }

        false
    }

    fn part_locations(&self) -> Vec<PartLocation> {
        (0..self.items.len())
            .flat_map(|y| {
                let row = &self.items[y];
                let mut x = 0;
                let mut row_items = vec![];

                while x < row.len() {
                    if row[x].is_ascii_digit() {
                        let number_start = x;
                        while row.get(x).map(|c| c.is_ascii_digit()).unwrap_or(false) {
                            x += 1;
                        }

                        let number: i32 = str::from_utf8(&row[number_start..x])
                            .unwrap()
                            .parse()
                            .unwrap();

                        if self.is_adjacent_to_symbol(number_start, x, y) {
                            row_items.push(PartLocation {
                                y,
                                x: number_start..x,
                                number,
                            });
                        }
                    }

                    x += 1;
                }

                row_items
            })
            .collect()
    }

    fn sum_of_parts(&self) -> i32 {
        self.part_locations()
            .iter()
            .map(|part| part.number)
            .sum::<i32>()
    }

    fn sum_of_gear_ratios(&self) -> i32 {
        let part_locations = self.part_locations();

        (0..self.items.len())
            .map(|y| {
                let row = &self.items[y];

                let mut cog_value = 0;

                for (x, &c) in row.iter().enumerate() {
                    if c != b'*' {
                        continue;
                    }

                    let y_range = y.saturating_sub(1)..=(y + 1);
                    let x_range = x.saturating_sub(1)..=(x + 1);

                    let gears = part_locations
                        .iter()
                        .filter_map(|part| {
                            if y_range.contains(&part.y)
                                && x_range.clone().any(|x| part.x.contains(&x))
                            {
                                Some(part.number)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    if gears.len() == 2 {
                        cog_value += gears[0] * gears[1];
                    }
                }

                cog_value
            })
            .sum::<i32>()
    }
}

#[test]
fn part1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let schematic = Schematic::parse(input);
    assert_eq!(schematic.sum_of_parts(), 4361);
}

#[test]
fn part2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    let schematic = Schematic::parse(input);
    assert_eq!(schematic.sum_of_gear_ratios(), 467835);
}
