use std::ops::{Add, Mul};

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "Part1: {}",
        part1(input, 200000000000000., 400000000000000.)
    );
}

struct Line {
    start_point: Point3d,
    velocity: Point3d,
}

#[derive(Clone, Copy, PartialEq)]
struct Point3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3d {
    fn parse(input: &str) -> Self {
        let pos = input
            .split(", ")
            .map(|text| text.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        Self {
            x: pos[0],
            y: pos[1],
            z: pos[2],
        }
    }

    fn project(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: 0.,
        }
    }

    fn distance_sq(self, other: &Self) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
    }
}

impl std::fmt::Debug for Point3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Point3d> for Point3d {
    type Output = Point3d;

    fn add(self, rhs: Point3d) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<f64> for Point3d {
    type Output = Point3d;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Line {
    fn parse(input: &str) -> Self {
        let (start_pos, velocity) = input.split_once(" @ ").unwrap();

        Self {
            start_point: Point3d::parse(start_pos),
            velocity: Point3d::parse(velocity),
        }
    }

    fn at(&self, t: f64) -> Point3d {
        self.start_point + self.velocity * t
    }

    fn intersect_2d(&self, other: &Line) -> Option<(f64, f64)> {
        let a1 = self.velocity.x;
        let b1 = self.velocity.y;
        let c1 = self.start_point.x;
        let d1 = self.start_point.y;

        let a2 = other.velocity.x;
        let b2 = other.velocity.y;
        let c2 = other.start_point.x;
        let d2 = other.start_point.y;

        let denominator = b2 * a1 - a2 * b1;
        if denominator.abs() < 0.00001 {
            return None;
        }

        let t1 = (b2 * (c2 - c1) - a2 * (d2 - d1)) as f64 / denominator as f64;
        let t2 = (b1 * (c1 - c2) - a1 * (d1 - d2)) as f64 / -denominator as f64;

        Some((t1, t2))
    }
}

fn part1(input: &str, start: f64, end: f64) -> usize {
    let lines = input
        .lines()
        .map(|line| Line::parse(line))
        .collect::<Vec<_>>();

    let mut collisions = 0;

    for (i, l1) in lines.iter().enumerate() {
        for l2 in lines.iter().skip(i + 1) {
            if let Some((t1, t2)) = l1.intersect_2d(l2) {
                if t1 < 0. || t2 < 0. {
                    continue;
                }

                let intersection_point = l1.at(t1);

                if start <= intersection_point.x
                    && intersection_point.x <= end
                    && start <= intersection_point.y
                    && intersection_point.y <= end
                {
                    collisions += 1;
                }
            }
        }
    }

    collisions
}

#[test]
fn intersection() {
    // 19, 13, 30 @ -2,  1, -2
    // 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @  1, -5, -3

    let l1 = Line {
        start_point: Point3d {
            x: 19.,
            y: 13.,
            z: 30.,
        },
        velocity: Point3d {
            x: -2.,
            y: 1.,
            z: -2.,
        },
    };

    let l2 = Line {
        start_point: Point3d {
            x: 18.,
            y: 19.,
            z: 22.,
        },
        velocity: Point3d {
            x: -1.,
            y: -1.,
            z: -2.,
        },
    };

    let (t1, t2) = l1.intersect_2d(&l2).unwrap();

    assert!(l1.at(t1).project().distance_sq(&l2.at(t2).project()) < 0.00001);
}

#[test]
fn part1_given_input() {
    assert_eq!(
        part1(
            "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3",
            7.,
            27.,
        ),
        2
    );
}
