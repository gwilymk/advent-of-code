pub use agb_fixnum::*;

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
