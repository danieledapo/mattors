//! Handy `Point` struct and utility functions.

extern crate num;
extern crate rand;

use std::convert::From;
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

/// Simple Triangle shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle<P>
where
    P: num::Num + From<u8> + Copy,
{
    /// The points of the triangle
    pub points: [Point<P>; 3],
}

impl<P> Triangle<P>
where
    P: num::Num + From<u8> + Copy,
{
    /// Create a new `Triangle` from the given points.
    pub fn new(p1: Point<P>, p2: Point<P>, p3: Point<P>) -> Triangle<P> {
        Triangle {
            points: [p1, p2, p3],
        }
    }

    /// Return the [centroid](https://en.wikipedia.org/wiki/Centroid) of the
    /// triangle.
    pub fn centroid(&self) -> Point<P> {
        let (sum_x, sum_y) = self
            .points
            .iter()
            .fold((P::zero(), P::zero()), |(accx, accy), pt| {
                (accx + pt.x, accy + pt.y)
            });

        let avg_x = sum_x / P::from(3);
        let avg_y = sum_y / P::from(3);

        Point::new(avg_x, avg_y)
    }
}

impl<P> Triangle<P>
where
    P: num::Num + num::Signed + From<u8> + Copy + PartialOrd,
{
    /// Return the area for this triangle.
    pub fn area(&self) -> P {
        self.signed_area().abs()
    }

    /// Return the signed area for this triangle. The sign indicates the
    /// orientation of the points. If it's negative then the vertices are in
    /// clockwise order, counter clockwise otherwise.
    pub fn signed_area(&self) -> P {
        let parallelogram_area = (self.points[1].x - self.points[0].x)
            * (self.points[2].y - self.points[0].y)
            - (self.points[2].x - self.points[0].x) * (self.points[1].y - self.points[0].y);

        parallelogram_area / P::from(2)
    }

    /// Transform this triangle so that the vertices are always in counter
    /// clockwise order.
    pub fn counter_clockwise(self) -> Self {
        if self.area() < P::from(0) {
            self
        } else {
            Triangle::new(
                self.points[1].clone(),
                self.points[0].clone(),
                self.points[2].clone(),
            )
        }
    }

    /// Return the circumcenter of the circle that encloses this triangle.
    pub fn circumcenter(&self) -> Option<Point<P>> {
        let p0p1 = LineEquation::between(&self.points[0], &self.points[1]);
        let p0p2 = LineEquation::between(&self.points[0], &self.points[2]);

        let mid_p0p1 = self.points[0].midpoint(&self.points[1]);
        let mid_p0p2 = self.points[0].midpoint(&self.points[2]);

        let bisec_p0p1 = p0p1.perpendicular(&mid_p0p1);
        let bisec_p0p2 = p0p2.perpendicular(&mid_p0p2);

        bisec_p0p1.intersection(&bisec_p0p2)
    }
}

impl<P> Triangle<P>
where
    P: num::Num + num::Signed + From<u8> + Copy + PartialOrd,
    f64: From<P>,
{
    /// Return the circumcicle that encloses this triangle as a pair of
    /// circumcenter and radius.
    pub fn circumcircle(&self) -> Option<(Point<P>, f64)> {
        self.circumcenter().map(|circumcenter| {
            let radius = circumcenter.dist(&self.points[0]);

            (circumcenter, radius)
        })
    }
}

/// Abstract representation of a line equation.
#[derive(Clone, Debug, PartialEq)]
pub enum LineEquation<T>
where
    T: num::Num + From<u8> + Copy,
{
    /// An equation for a `VerticalLine` in the given x coordinate.
    VerticalLine(T),

    /// A non vertical line equation.
    Line {
        /// The slope of the line.
        slope: T,

        /// The interception of the line with the y axis.
        yintercept: T,
    },
}

impl<T> LineEquation<T>
where
    T: num::Signed + From<u8> + Copy,
{
    /// Build a new `LineEquation` that represents a line intersecting both of
    /// the given points.
    pub fn between(p1: &Point<T>, p2: &Point<T>) -> Self {
        if let Some(slope) = p1.slope(p2) {
            let yintercept = p1.yintercept(slope);

            Self::line(slope, yintercept)
        } else {
            Self::vertical(p1.x)
        }
    }

    /// Build a `LineEquation` for a vertical line.
    pub fn vertical(x: T) -> Self {
        LineEquation::VerticalLine(x)
    }

    /// Build a `LineEquation` for an horizontal line.
    pub fn horizonal(y: T) -> Self {
        LineEquation::Line {
            slope: T::from(0),
            yintercept: y,
        }
    }

    /// Build a `LineEquation` with the given `slope` and `yintercept`.
    pub fn line(slope: T, yintercept: T) -> Self {
        LineEquation::Line { slope, yintercept }
    }

    /// Calculate the y coordinate at the given x. Returns `None` if it's a
    /// `VerticalLine`.
    pub fn y_at(&self, x: T) -> Option<T> {
        match *self {
            LineEquation::VerticalLine(_) => None,
            LineEquation::Line { slope, yintercept } => Some(slope * x + yintercept),
        }
    }

    /// Calculate the intersection point between two lines. Returns `None` if
    /// the lines are parallel. **Note**: this method returns `None` even when
    /// `self` and `other` are the same `VerticalLine`.
    pub fn intersection(&self, other: &Self) -> Option<Point<T>> {
        // FIXME: might want to return an IntersectionResult enum composed by:
        // - NoIntersection
        // - SameVerticalLine(x)
        // - Point(p)
        // but it's probably overkill for now.
        use self::LineEquation::{Line, VerticalLine};

        match (self, other) {
            (VerticalLine(_), VerticalLine(_)) => None,
            (VerticalLine(x), l) | (l, VerticalLine(x)) => {
                Some(Point::new(*x, l.y_at(*x).unwrap()))
            }
            (
                Line {
                    slope: slope1,
                    yintercept: c1,
                },
                Line {
                    slope: slope2,
                    yintercept: c2,
                },
            ) => {
                if slope1 != slope2 {
                    let x = (*c2 - *c1) / (*slope1 - *slope2);
                    let y = self.y_at(x).unwrap();

                    Some(Point::new(x, y))
                } else {
                    None
                }
            }
        }
    }

    /// Return the perpendicular line to this line that intersects the given
    /// point.
    pub fn perpendicular(&self, p: &Point<T>) -> Self {
        use self::LineEquation::{Line, VerticalLine};

        match self {
            &VerticalLine(_) => Self::horizonal(p.y),
            &Line { slope, .. } => {
                if slope == T::from(0) {
                    Self::vertical(p.x)
                } else {
                    let perp_slope = -T::from(1) / slope;
                    let yintercept = p.yintercept(perp_slope);

                    Self::line(perp_slope, yintercept)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{LineEquation, PointF64, PointI32, PointU32, Triangle};

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

    #[test]
    fn test_triangle_circumcircle() {
        let triangle = Triangle::new(
            PointI32::new(3, 2),
            PointI32::new(1, 4),
            PointI32::new(5, 4),
        );
        assert_eq!(triangle.circumcircle(), Some((PointI32::new(3, 4), 2.0)));

        // ensure the algorithm works with vertical lines
        let triangle = Triangle::new(
            PointI32::new(3, 2),
            PointI32::new(5, 4),
            PointI32::new(1, 4),
        );
        assert_eq!(triangle.circumcircle(), Some((PointI32::new(3, 4), 2.0)));

        let triangle = Triangle::new(
            PointI32::new(3, 2),
            PointI32::new(5, 2),
            PointI32::new(4, 2),
        );
        assert_eq!(triangle.circumcircle(), None);
    }

    #[test]
    fn test_triangle_area() {
        let triangle = Triangle::new(
            PointI32::new(6, 0),
            PointI32::new(0, 0),
            PointI32::new(3, 3),
        );
        assert_eq!(triangle.area(), 9);
        assert_eq!(triangle.signed_area(), -9);

        let triangle = triangle.counter_clockwise();
        assert_eq!(triangle.area(), 9);
        assert_eq!(triangle.signed_area(), 9);
    }

    #[test]
    fn test_line_between() {
        assert_eq!(
            LineEquation::between(&PointI32::new(3, 3), &PointI32::new(9, 9)),
            LineEquation::line(1, 0)
        );

        assert_eq!(
            LineEquation::between(&PointI32::new(3, 5), &PointI32::new(-3, -1)),
            LineEquation::line(1, 2)
        );

        assert_eq!(
            LineEquation::between(&PointI32::new(1, -10), &PointI32::new(-3, 22)),
            LineEquation::line(-8, -2)
        );

        assert_eq!(
            LineEquation::between(&PointI32::new(1, -10), &PointI32::new(1, 22)),
            LineEquation::vertical(1)
        );
    }

    #[test]
    fn test_line_yat() {
        let line1 = LineEquation::line(1, 5);

        assert_eq!(line1.y_at(0), Some(5));
        assert_eq!(line1.y_at(3), Some(8));
        assert_eq!(line1.y_at(-4), Some(1));

        let line2 = LineEquation::line(-3, -5);

        assert_eq!(line2.y_at(0), Some(-5));
        assert_eq!(line2.y_at(3), Some(-14));
        assert_eq!(line2.y_at(-4), Some(7));

        let vertical = LineEquation::vertical(2);
        assert_eq!(vertical.y_at(7), None);
        assert_eq!(vertical.y_at(2), None);
    }

    #[test]
    fn test_line_intersection() {
        assert_eq!(
            LineEquation::vertical(2).intersection(&LineEquation::vertical(5)),
            None
        );

        let line1 = LineEquation::line(1, 4);

        assert_eq!(
            LineEquation::vertical(2).intersection(&line1),
            Some(PointI32::new(2, 6))
        );

        // parallel line
        assert_eq!(line1.intersection(&LineEquation::line(1, -42)), None);

        let line2 = LineEquation::line(-1, -2);
        assert_eq!(line1.intersection(&line2), Some(PointI32::new(-3, 1)));
    }

    #[test]
    fn test_line_perpendicular() {
        assert_eq!(
            LineEquation::vertical(4).perpendicular(&PointI32::new(3, 2)),
            LineEquation::horizonal(2)
        );

        assert_eq!(
            LineEquation::horizonal(2).perpendicular(&PointI32::new(1, 4)),
            LineEquation::vertical(1)
        );

        let origin = PointI32::new(0, 0);
        let bisec1 = LineEquation::line(1, 0);
        let bisec2 = LineEquation::line(-1, 0);

        assert_eq!(bisec1.perpendicular(&origin), bisec2);
        assert_eq!(bisec2.perpendicular(&origin), bisec1);

        let p = PointI32::new(0, 3);
        let line1 = LineEquation::line(1, 3);
        let line2 = LineEquation::line(-1, 3);

        assert_eq!(line1.perpendicular(&p), line2);
        assert_eq!(line2.perpendicular(&p), line1);
    }
}
