use std::{env, fs, path::PathBuf};

pub use agb_fixnum::*;
use anyhow::Context;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub struct Grid2<T> {
    pub points: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid2<T> {
    pub fn parse(input: &str, line_mapper: impl Fn(&str) -> Vec<T>) -> Self {
        let points = input.split('\n').map(line_mapper).collect::<Vec<_>>();

        let width = points[0].len();
        assert!(
            points.iter().all(|line| line.len() == width),
            "All lines must be the same width"
        );

        let height = points.len();

        Self {
            points,
            width,
            height,
        }
    }

    pub fn get<V: TryInto<usize> + agb_fixnum::FixedWidthUnsignedInteger>(
        &self,
        point: impl Into<Vector2D<V>>,
    ) -> Option<&T> {
        let point = point.into();

        let x = point.x.try_into().ok()?;
        let y = point.y.try_into().ok()?;

        self.points.get(y)?.get(x)
    }

    pub fn neighbours_with_points<V: TryInto<i32> + agb_fixnum::FixedWidthUnsignedInteger>(
        &self,
        point: impl Into<Vector2D<V>>,
        include_diagonals: bool,
    ) -> impl Iterator<Item = (&T, Vector2D<i32>)> {
        let point = point.into();

        (-1..=1).flat_map(move |y| {
            (-1..=1).filter_map(move |x| {
                if !include_diagonals && x * y != 0 {
                    return None;
                }

                if x == 0 && y == 0 {
                    return None;
                }

                let cx = point.x.try_into();
                let cy = point.y.try_into();

                match (cx, cy) {
                    (Ok(cx), Ok(cy)) => {
                        let px = cx + x;
                        let py = cy + y;

                        Some((self.get::<i32>((px, py))?, Vector2D::new(px, py)))
                    }
                    _ => None,
                }
            })
        })
    }

    pub fn neighbours<V: TryInto<i32> + agb_fixnum::FixedWidthUnsignedInteger>(
        &self,
        point: impl Into<Vector2D<V>>,
        include_diagonals: bool,
    ) -> impl Iterator<Item = &T> {
        self.neighbours_with_points(point, include_diagonals)
            .map(|(n, _)| n)
    }
}

pub trait AllPairsExt<Item> {
    fn all_pairs<'a>(&'a self) -> impl Iterator<Item = (Item, Item)> + 'a
    where
        Item: 'a;
}

impl<T, U> AllPairsExt<T> for U
where
    U: AsRef<[T]>,
    T: Clone + 'static,
{
    fn all_pairs<'a>(&'a self) -> impl Iterator<Item = (T, T)> + 'a
    where
        T: 'a,
    {
        let slice: &[T] = self.as_ref();
        slice.iter().enumerate().flat_map(move |(i, x)| {
            slice
                .iter()
                .skip(i + 1)
                .map(move |y| (x.clone(), y.clone()))
        })
    }
}

pub fn get_input(day: i32) -> String {
    try_get_input(day).unwrap()
}

pub fn try_get_input(day: i32) -> anyhow::Result<String> {
    let exe_path = env::current_exe()?;
    let exe_dir = exe_path.parent().unwrap();

    let cached_input = exe_dir.with_file_name(format!("input-{day}.txt"));
    if cached_input.exists() {
        return fs::read_to_string(cached_input).context("Failed to read input for today");
    }

    println!("Fetching input for day {day}");

    let home = env::var("HOME")?;
    let access_cookie = fs::read_to_string(PathBuf::from(home).join(".aoc-cookie"))
        .context("Could not read aoc cookie")?;

    let client = reqwest::blocking::Client::new();
    let input = client
        .request(
            reqwest::Method::GET,
            format!("https://adventofcode.com/2024/day/{day}/input"),
        )
        .header("Cookie", format!("session={}", access_cookie.trim()))
        .send()?
        .text()?;

    let input = input.trim().to_string();

    fs::write(&cached_input, &input)?;

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid2_get_points() {
        let grid = Grid2::parse("abc\ndef\nghi", |line| line.chars().collect());

        assert_eq!(grid.get::<i32>((1, 1)), Some(&'e'));
        assert_eq!(grid.get::<i32>((1, 4)), None);
        assert_eq!(grid.get::<i32>((-1, -2)), None);
    }

    #[test]
    fn grid2_get_neighbours() {
        let grid = Grid2::parse("123\n456\n789", |line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect()
        });

        let non_diagonal = grid.neighbours::<i32>((0, 0), false).collect::<Vec<_>>();
        assert_eq!(non_diagonal, &[&2, &4]);

        let diagonal = grid.neighbours::<i32>((0, 0), true).collect::<Vec<_>>();
        assert_eq!(diagonal, &[&2, &4, &5]);
    }

    #[test]
    fn all_pairs() {
        let test_input = &[1, 2, 3, 4, 5];
        let pairs = test_input.all_pairs().collect::<Vec<_>>();

        assert_eq!(
            pairs,
            &[
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (2, 3),
                (2, 4),
                (2, 5),
                (3, 4),
                (3, 5),
                (4, 5)
            ]
        )
    }
}
