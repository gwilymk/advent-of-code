use std::collections::HashSet;

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(12);
    let garden = Garden::new(&input);
    println!("Part 1: {}", garden.fence_costs());
}

struct Garden {
    map: Grid2<char>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let map = Grid2::parse(input, |line| line.chars().collect());
        Self { map }
    }

    fn fence_costs(&self) -> usize {
        let mut explored = HashSet::new();
        let mut cost = 0;

        for (point, plant) in self.map.iter() {
            if !explored.insert(point) {
                continue;
            }

            let this_region = self.get_plant_region(*plant, point);

            explored.extend(this_region.iter());

            let area = this_region.len();
            let mut perimeter = 0;

            for point in this_region.iter() {
                for y in -1..=1 {
                    for x in -1..=1 {
                        if !((x == 0) ^ (y == 0)) {
                            continue;
                        }

                        let search_point = *point + Vector2D::new(x, y);
                        if !this_region.contains(&search_point) {
                            perimeter += 1;
                        }
                    }
                }
            }

            cost += perimeter * area;
        }

        cost
    }

    fn get_plant_region(&self, plant: char, point: Vector2D<i32>) -> HashSet<Vector2D<i32>> {
        let mut result = HashSet::new();
        result.insert(point);
        self.get_plant_region_search(plant, point, &mut result);
        result
    }

    fn get_plant_region_search(
        &self,
        plant: char,
        point: Vector2D<i32>,
        result: &mut HashSet<Vector2D<i32>>,
    ) {
        for (neighbour, neighbour_point) in self.map.neighbours_with_points::<i32>(point, false) {
            if *neighbour != plant {
                continue;
            }

            if !result.insert(neighbour_point) {
                continue;
            }

            self.get_plant_region_search(plant, neighbour_point, result);
        }
    }
}

#[test]
fn given_input() {
    let garden = Garden::new(
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    );

    assert_eq!(garden.fence_costs(), 1930);
}
