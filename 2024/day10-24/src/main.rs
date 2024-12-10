use std::collections::HashSet;

fn main() {
    let map = Map::parse(include_str!("input.txt"));
    println!("part 1: {}", map.total_trailheads());
}

struct Map {
    heights: Vec<Vec<u32>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let heights = input
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self { heights }
    }

    fn total_trailheads(&self) -> u32 {
        self.heights
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, n)| {
                        if *n != 0 {
                            return None;
                        }

                        let mut results = HashSet::new();

                        self.trailheads_starting_at_point(x, y, &mut results);
                        Some(results.len() as u32)
                    })
                    .sum::<u32>()
            })
            .sum()
    }

    fn trailheads_starting_at_point(
        &self,
        start_x: usize,
        start_y: usize,
        results: &mut HashSet<(usize, usize)>,
    ) {
        let height = self.heights[start_y][start_x];

        if height == 9 {
            results.insert((start_x, start_y));
            return;
        }

        if start_x > 0 && self.heights[start_y][start_x - 1] == height + 1 {
            self.trailheads_starting_at_point(start_x - 1, start_y, results);
        }

        if start_y > 0 && self.heights[start_y - 1][start_x] == height + 1 {
            self.trailheads_starting_at_point(start_x, start_y - 1, results);
        }

        if start_x < self.heights[0].len() - 1 && self.heights[start_y][start_x + 1] == height + 1 {
            self.trailheads_starting_at_point(start_x + 1, start_y, results);
        }

        if start_y < self.heights.len() - 1 && self.heights[start_y + 1][start_x] == height + 1 {
            self.trailheads_starting_at_point(start_x, start_y + 1, results);
        }
    }
}

#[test]
fn given_input() {
    let map = Map::parse(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    );
    assert_eq!(map.total_trailheads(), 36);
}
