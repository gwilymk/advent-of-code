use std::mem;

pub use agb_fixnum::*;

mod input;
pub use input::*;

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

    pub fn set<V: TryInto<usize> + agb_fixnum::FixedWidthUnsignedInteger>(
        &mut self,
        point: impl Into<Vector2D<V>>,
        mut value: T,
    ) -> Option<T> {
        let point = point.into();

        let x = point.x.try_into().ok()?;
        let y = point.y.try_into().ok()?;

        let current_value = self.points.get_mut(y)?.get_mut(x)?;
        mem::swap(current_value, &mut value);
        Some(value)
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

    pub fn iter(&self) -> impl Iterator<Item = (Vector2D<i32>, &'_ T)> + '_
    where
        T: Clone,
    {
        self.points.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, value)| (Vector2D::new(x as i32, y as i32), value))
        })
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

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

/// Returns x, y, z such that:
///
/// a * x + b * y = gcd(a, b) = z
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }

    (old_s, old_t, old_r)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn parse(c: char) -> Option<Self> {
        Some(match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => return None,
        })
    }

    pub fn neighbours(self) -> [Self; 2] {
        match self {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }

    pub fn all() -> [Direction; 4] {
        use Direction::*;
        [North, East, South, West]
    }

    pub fn rotate_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl From<&Direction> for Vector2D<i32> {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
        .into()
    }
}

impl From<Direction> for Vector2D<i32> {
    fn from(value: Direction) -> Self {
        Vector2D::from(&value)
    }
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

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(5, 7), 1);
        assert_eq!(gcd(15, 21), 3);
    }

    #[test]
    fn extended_gcd_test() {
        let (x, y, z) = extended_gcd(240, 46);

        assert_eq!(x * 240 + y * 46, z);
        assert_eq!(z, gcd(240, 46) as i64);
    }
}
