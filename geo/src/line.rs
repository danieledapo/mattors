//! Simple module to work with lines.

use crate::point::Point;

/// Linearly interpolate the point at the given x using the line that goes
/// between the two points.
pub fn linear_interpolate<T>(p1: &Point<T>, p2: &Point<T>, x: T) -> Option<T>
where
    T: num::Num + From<u8> + Copy,
{
    let line = LineEquation::between(p1, p2);

    line.y_at(x)
}

/// Abstract representation of a line equation.
#[derive(Clone, Debug, PartialEq)]
pub enum LineEquation<T> {
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
    T: num::Num + From<u8> + Copy,
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

    /// Calculate the x coordinate at the given y. Returns `None` if it's a
    /// horizontal line.
    pub fn x_at(&self, y: T) -> Option<T> {
        match *self {
            LineEquation::VerticalLine(x) => Some(x),
            LineEquation::Line { slope, yintercept } => {
                if slope == T::from(0) {
                    None
                } else {
                    Some((y - yintercept) / slope)
                }
            }
        }
    }

    /// Return the delta x that corresponds to the given dy. `None` if the line
    /// is horizontal.
    pub fn dx_for(&self, dy: T) -> Option<T> {
        match *self {
            LineEquation::VerticalLine(_) => Some(T::from(0)),
            LineEquation::Line { slope, .. } => {
                if slope == T::from(0) {
                    None
                } else {
                    Some(dy / slope)
                }
            }
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
}

impl<T> LineEquation<T>
where
    T: num::Signed + From<u8> + Copy,
{
    /// Return the perpendicular line to this line that intersects the given
    /// point.
    pub fn perpendicular(&self, p: &Point<T>) -> Self {
        use self::LineEquation::{Line, VerticalLine};

        match *self {
            VerticalLine(_) => Self::horizonal(p.y),
            Line { slope, .. } => {
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
    use super::LineEquation;
    use geo::PointI32;

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
    fn test_line_xat() {
        let line1 = LineEquation::between(&PointI32::new(3, 4), &PointI32::new(9, 10));
        assert_eq!(line1.x_at(4), Some(3));
        assert_eq!(line1.x_at(10), Some(9));
        assert_eq!(line1.x_at(1), Some(0));

        let horizontal = LineEquation::horizonal(73);
        assert_eq!(horizontal.x_at(0), None);
        assert_eq!(horizontal.x_at(3), None);
        assert_eq!(horizontal.x_at(-4), None);

        let vertical = LineEquation::vertical(2);
        assert_eq!(vertical.x_at(0), Some(2));
        assert_eq!(vertical.x_at(-42), Some(2));
    }

    #[test]
    fn test_line_dx_for() {
        let line1 = LineEquation::between(&PointI32::new(3, 4), &PointI32::new(9, 10));
        assert_eq!(line1.dx_for(5), Some(5));
        assert_eq!(line1.dx_for(-3), Some(-3));
        assert_eq!(line1.dx_for(0), Some(0));

        let horizontal = LineEquation::horizonal(73);
        assert_eq!(horizontal.dx_for(0), None);
        assert_eq!(horizontal.dx_for(3), None);
        assert_eq!(horizontal.dx_for(-4), None);

        let vertical = LineEquation::vertical(2);
        assert_eq!(vertical.dx_for(0), Some(0));
        assert_eq!(vertical.dx_for(-42), Some(0));
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
