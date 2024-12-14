use std::sync::LazyLock;

use aoc2024::{get_input, Vector2D};
use regex::Regex;

fn main() {
    let input = get_input(14);
    println!("Part 1: {}", part1(&input, 101, 103));

    // just need to see it
    part2(&input, 101, 103);
}

struct Robot {
    start_point: Vector2D<i32>,
    velocity: Vector2D<i32>,
}

impl Robot {
    fn final_location(&self, seconds: u32, width: i32, height: i32) -> Vector2D<i32> {
        let distance_travelled = self.velocity * seconds as i32;
        let final_location = self.start_point + distance_travelled;
        (
            final_location.x.rem_euclid(width),
            final_location.y.rem_euclid(height),
        )
            .into()
    }

    fn parse(line: &str) -> Self {
        static LINE_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap());

        let (_, [p0, p1, v0, v1]) = LINE_REGEX.captures(line).unwrap().extract();

        Self {
            start_point: Vector2D::new(p0.parse().unwrap(), p1.parse().unwrap()),
            velocity: Vector2D::new(v0.parse().unwrap(), v1.parse().unwrap()),
        }
    }
}

fn part1(input: &str, width: i32, height: i32) -> u32 {
    let robots = input.split('\n').map(Robot::parse);

    let final_locations = robots.map(|robot| robot.final_location(100, width, height));

    let mut quadrants = [0, 0, 0, 0];
    for final_location in final_locations {
        if final_location.x < width / 2 && final_location.y < height / 2 {
            quadrants[0] += 1;
        } else if final_location.x > width / 2 && final_location.y < height / 2 {
            quadrants[1] += 1;
        } else if final_location.x < width / 2 && final_location.y > height / 2 {
            quadrants[2] += 1;
        } else if final_location.x > width / 2 && final_location.y > height / 2 {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().product()
}

fn part2(input: &str, width: i32, height: i32) {
    let robots = input.split('\n').map(Robot::parse).collect::<Vec<_>>();
    let mut lowest_factor = i32::MAX;

    for time in 0.. {
        let final_locations = robots
            .iter()
            .map(|robot| robot.final_location(time, width, height));

        let mut quadrants = [0, 0, 0, 0];
        let mut map = vec![false; (width * height) as usize];

        for final_location in final_locations {
            map[(final_location.x + final_location.y * width) as usize] = true;

            if final_location.x < width / 2 && final_location.y < height / 2 {
                quadrants[0] += 1;
            } else if final_location.x > width / 2 && final_location.y < height / 2 {
                quadrants[1] += 1;
            } else if final_location.x < width / 2 && final_location.y > height / 2 {
                quadrants[2] += 1;
            } else if final_location.x > width / 2 && final_location.y > height / 2 {
                quadrants[3] += 1;
            }
        }

        let factor = quadrants.iter().product::<i32>();

        if factor < lowest_factor {
            lowest_factor = factor;
            println!("{time} {factor}");
        } else {
            continue;
        }

        for y in 0..height {
            for x in 0..width {
                if map[(x + y * width) as usize] {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }
}

#[test]
fn given_input() {
    assert_eq!(
        part1(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            11,
            7
        ),
        12
    );
}
