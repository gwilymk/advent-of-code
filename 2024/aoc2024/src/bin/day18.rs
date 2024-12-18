use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc2024::{get_input, Grid2, Vector2D};

fn main() {
    let input = get_input(18);
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let ram = Ram::new(input, 71, 71);
    ram.path_to_exit(1024).len()
}

struct Ram {
    map: Grid2<usize>,
}

impl Ram {
    fn new(input: &str, width: usize, height: usize) -> Self {
        let mut map = Grid2::new_with(width, height, || usize::MAX);
        for (time, line) in input.split('\n').enumerate() {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();

            map.set::<i32>((x, y), time);
        }

        Self { map }
    }

    fn path_to_exit(&self, max_value: usize) -> Vec<Vector2D<i32>> {
        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Node {
            distance: usize,
            point: Vector2D<i32>,
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.distance.cmp(&other.distance)
            }
        }

        let start = Vector2D::new(0, 0);
        let end = Vector2D::new((self.map.width - 1) as i32, (self.map.height - 1) as i32);

        let mut q = BinaryHeap::new();
        let mut distance: HashMap<Vector2D<i32>, usize> = HashMap::new();

        let mut previous: HashMap<Vector2D<i32>, Vector2D<i32>> = HashMap::new();

        q.push(Reverse(Node {
            distance: 0,
            point: start,
        }));

        while let Some(Reverse(minimum)) = q.pop() {
            if minimum.point == end {
                break;
            }

            for (&neighbour_value, neighbour_point) in
                self.map.neighbours_with_points::<i32>(minimum.point, false)
            {
                if neighbour_value < max_value {
                    // part 1, anything over 1024 is considered filled
                    continue;
                }

                let neighbour_distance = minimum.distance + 1;

                let current_distance = *distance.get(&neighbour_point).unwrap_or(&usize::MAX);
                if neighbour_distance < current_distance {
                    previous.insert(neighbour_point, minimum.point);
                    distance.insert(neighbour_point, neighbour_distance);
                    q.push(Reverse(Node {
                        distance: neighbour_distance,
                        point: neighbour_point,
                    }));
                }
            }
        }

        let mut result = vec![end];
        while result[result.len() - 1] != start {
            result.push(previous[&result[result.len() - 1]]);
        }

        result
    }
}

#[test]
fn given_input() {
    let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    let ram = Ram::new(input, 7, 7);

    assert_eq!(ram.path_to_exit(12).len() - 1, 22);
}
