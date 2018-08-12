//! Handy `Point` struct and utility functions.

extern crate num;

use std::error::Error;
use std::str::FromStr;

/// Point specialized for `f64`
pub type PointF64 = Point<f64>;

/// Point specialized for `i32`
pub type PointI32 = Point<i32>;

/// Point specialized for `u32`
pub type PointU32 = Point<u32>;

/// Simple 2d Point struct
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point<T>
where
    T: num::Num + From<u8> + Copy,
{
    /// x coordinate
    pub x: T,

    /// y coordinate
    pub y: T,
}

impl<T> Point<T>
where
    T: num::Num + From<u8> + Copy,
{
    /// Create a new `Point` with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    /// Calculate the midpoint between this point and another.
    pub fn midpoint(&self, p: &Self) -> Self {
        Point::new((self.x + p.x) / T::from(2), (self.y + p.y) / T::from(2))
    }

    /// Calculate the slope between this point and another. Return `None` if
    /// the slope is undefined that is when `self` and `p` form a vertical line.
    pub fn slope<O>(&self, p: &Self) -> Option<O>
    where
        O: num::Signed + From<T>,
    {
        if self.x == p.x {
            None
        } else {
            let dx = O::from(self.x) - O::from(p.x);
            let dy = O::from(self.y) - O::from(p.y);

            Some(dy / dx)
        }
    }

    /// Calculate the y-intercept of line that has the given `slope` and that
    /// intersects with this point.
    pub fn yintercept(&self, slope: T) -> T {
        self.y - slope * self.x
    }

    /// Calculate the squared distance between this point and another.
    pub fn dist<O>(&self, p: &Self) -> O
    where
        O: num::Float + From<T>,
    {
        self.squared_dist::<O>(p).sqrt()
    }

    /// Calculate the distance between this point and another.
    pub fn squared_dist<O>(&self, p: &Self) -> O
    where
        O: num::Num + From<T> + Copy,
    {
        let dx = <O as From<T>>::from(self.x) - <O as From<T>>::from(p.x);
        let dy = <O as From<T>>::from(self.y) - <O as From<T>>::from(p.y);

        dx * dx + dy * dy
    }

    /// Return a copy of this point with different types.
    pub fn cast<O>(&self) -> Point<O>
    where
        O: num::Num + From<T> + From<u8> + Copy,
    {
        Point::new(O::from(self.x), O::from(self.y))
    }
}

impl<T> Point<T>
where
    T: num::Num + From<u8> + Copy + Ord,
{
    /// Handy method that returns a point that's composed by the highest x and y
    /// values among the two points.
    pub fn top_right(&self, other: &Self) -> Self {
        Point::new(self.x.max(other.x), self.y.max(other.y))
    }

    /// Handy method that returns a point that's composed by the lowest x and y
    /// values among the two points.
    pub fn bottom_left(&self, other: &Self) -> Self {
        Point::new(self.x.min(other.x), self.y.min(other.y))
    }
}

impl<T: FromStr> FromStr for Point<T>
where
    T: num::Num + From<u8> + Copy,
    T::Err: Error,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Result<Vec<T>, T::Err> = s.trim().split(',').map(|p| p.parse()).collect();

        match points {
            Err(e) => Err("bad coord number format, ".to_string() + e.description()),
            Ok(points) => {
                if points.len() != 2 {
                    Err("wrong number of coords, please pass x and y coords as floats separated by ','".to_string())
                } else {
                    Ok(Self {
                        x: points[0],
                        y: points[1],
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{PointF64, PointI32, PointU32};

    #[test]
    fn test_midpoint() {
        assert_eq!(
            PointU32::new(0, 0).midpoint(&PointU32::new(6, 6)),
            PointU32::new(3, 3)
        );

        assert_eq!(
            PointU32::new(4, 6).midpoint(&PointU32::new(2, 8)),
            PointU32::new(3, 7)
        );

        assert_eq!(
            PointF64::new(-4.0, 6.0).midpoint(&PointF64::new(8.0, -8.0)),
            PointF64::new(2.0, -1.0)
        );

        assert_eq!(
            PointF64::new(0.0, 0.0).midpoint(&PointF64::new(-10.0, -4.0)),
            PointF64::new(-5.0, -2.0)
        );
    }

    #[test]
    fn test_slope() {
        assert_eq!(PointU32::new(1, 1).slope(&PointU32::new(3, 3)), Some(1_i64));
        assert_eq!(PointU32::new(3, 3).slope(&PointU32::new(1, 1)), Some(1_i64));

        assert_eq!(PointU32::new(0, 1).slope(&PointU32::new(3, 7)), Some(2_i64));
        assert_eq!(PointU32::new(3, 7).slope(&PointU32::new(0, 1)), Some(2_i64));

        assert_eq!(
            PointU32::new(0, 8).slope(&PointU32::new(8, 0)),
            Some(-1_i64)
        );
        assert_eq!(
            PointU32::new(8, 0).slope(&PointU32::new(0, 8)),
            Some(-1_i64)
        );

        assert_eq!(
            PointU32::new(2, 11).slope(&PointU32::new(7, 0)),
            Some(-2_i64)
        );
        assert_eq!(
            PointU32::new(7, 0).slope(&PointU32::new(2, 11)),
            Some(-2_i64)
        );

        // horizontal
        assert_eq!(
            PointU32::new(0, 7).slope(&PointU32::new(42, 7)),
            Some(0_i64)
        );
        assert_eq!(
            PointU32::new(42, 7).slope(&PointU32::new(0, 7)),
            Some(0_i64)
        );

        // vertical
        assert_eq!(
            PointU32::new(7_, 0).slope::<i64>(&PointU32::new(7, 53)),
            None
        );
        assert_eq!(
            PointU32::new(7, 53).slope::<i64>(&PointU32::new(7, 0)),
            None
        );
    }

    #[test]
    fn test_yintercept() {
        assert_eq!(PointU32::new(0, 0).yintercept(1), 0);
        assert_eq!(PointU32::new(2, 12).yintercept(2), 8);

        assert_eq!(PointI32::new(2, 12).yintercept(-2), 16);
        assert_eq!(PointI32::new(-4, 30).yintercept(-8), -2);
    }

    #[test]
    fn test_top_right() {
        assert_eq!(
            PointU32::new(0, 0).top_right(&PointU32::new(4, 10)),
            PointU32::new(4, 10)
        );
        assert_eq!(
            PointU32::new(10, 0).top_right(&PointU32::new(4, 10)),
            PointU32::new(10, 10)
        );
        assert_eq!(
            PointU32::new(0, 12).top_right(&PointU32::new(4, 10)),
            PointU32::new(4, 12)
        );
        assert_eq!(
            PointU32::new(10, 12).top_right(&PointU32::new(4, 10)),
            PointU32::new(10, 12)
        );
    }

    #[test]
    fn test_bottom_left() {
        assert_eq!(
            PointU32::new(0, 0).bottom_left(&PointU32::new(4, 10)),
            PointU32::new(0, 0)
        );
        assert_eq!(
            PointU32::new(10, 0).bottom_left(&PointU32::new(4, 10)),
            PointU32::new(4, 0)
        );
        assert_eq!(
            PointU32::new(0, 12).bottom_left(&PointU32::new(4, 10)),
            PointU32::new(0, 10)
        );
        assert_eq!(
            PointU32::new(10, 12).bottom_left(&PointU32::new(4, 10)),
            PointU32::new(4, 10)
        );
    }

    #[test]
    fn test_dist() {
        let origin = PointI32::new(0, 0);

        let p1 = PointI32::new(0, 4);
        assert_eq!(origin.dist::<f64>(&p1), 4.0);
        assert_eq!(p1.dist::<f64>(&origin), 4.0);

        let p2 = PointI32::new(3, 0);
        assert_eq!(origin.dist::<f64>(&p2), 3.0);
        assert_eq!(p2.dist::<f64>(&origin), 3.0);

        assert_eq!(p2.dist::<f64>(&p1), 5.0);
        assert_eq!(p1.dist::<f64>(&p2), 5.0);

        assert_eq!(origin.dist::<f64>(&PointI32::new(-4, 0)), 4.0);
        assert_eq!(PointI32::new(0, -4).dist::<f64>(&origin), 4.0);

        assert_eq!(PointI32::new(3, 5).dist::<f64>(&PointI32::new(6, 9)), 5.0);
        assert_eq!(PointI32::new(6, 9).dist::<f64>(&PointI32::new(3, 5)), 5.0);
    }
}
