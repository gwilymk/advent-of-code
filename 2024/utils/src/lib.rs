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

    pub fn neighbours<V: TryInto<usize> + agb_fixnum::FixedWidthUnsignedInteger>(
        &self,
        point: impl Into<Vector2D<V>>,
        include_diagonals: bool,
    ) -> impl Iterator<Item = &T> {
        let point = point.into();
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
    }
}
