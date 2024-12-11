use std::collections::HashSet;

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let map = Map::parse(&get_input(10));
    let result = map.total_trailheads();
    println!("part 1: {}", result.0);
    println!("part 2: {}", result.1);
}

struct Map {
    heights: Grid2<u32>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let heights = Grid2::parse(input, |line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        });

        Self { heights }
    }

    // total trail heads, total rating
    fn total_trailheads(&self) -> (u32, u32) {
        self.heights
            .iter()
            .filter_map(|(point, n)| {
                if *n != 0 {
                    return None;
                }

                let mut results = HashSet::new();

                let rating = self.trailheads_starting_at_point(point, &mut results);
                Some((results.len() as u32, rating))
            })
            .fold((0, 0), |acc, next| (acc.0 + next.0, acc.1 + next.1))
    }

    // returns the rating
    fn trailheads_starting_at_point(
        &self,
        point: Vector2D<i32>,
        results: &mut HashSet<Vector2D<i32>>,
    ) -> u32 {
        let height = *self.heights.get::<i32>(point).unwrap();
        let mut total = 0;

        if height == 9 {
            results.insert(point);
            return 1;
        }

        for (value, location) in self.heights.neighbours_with_points::<i32>(point, false) {
            if *value == height + 1 {
                total += self.trailheads_starting_at_point(location, results);
            }
        }

        total
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
    assert_eq!(map.total_trailheads(), (36, 81));
}
