//! Module that contains simple utilities to work with angles.

use std::cmp::Ordering;

use point::Point;
use utils::cmp_floats;

/// The orientation of an angle, for example between three points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleOrientation {
    /// Counter-clockwise direction
    CounterClockwise,

    /// Clockwsise direction
    Clockwise,

    /// Colinear direction
    Colinear,
}

/// Calculate the polar angle between the two points.
pub fn polar_angle(p1: &Point<f64>, p2: &Point<f64>) -> f64 {
    f64::atan2(p2.y - p1.y, p2.x - p1.x)
}

/// Calculate the angle orientation between three points where p2 is the center
/// point.
pub fn angle_orientation(p1: &Point<f64>, p2: &Point<f64>, p3: &Point<f64>) -> AngleOrientation {
    let area = (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x);

    match cmp_floats(area, 0.0) {
        Ordering::Equal => AngleOrientation::Colinear,
        Ordering::Less => AngleOrientation::Clockwise,
        Ordering::Greater => AngleOrientation::CounterClockwise,
    }
}

#[cfg(test)]
mod tests {
    use super::{angle_orientation, AngleOrientation};

    use geo::PointF64;

    #[test]
    fn test_angle_orientation() {
        assert_eq!(
            angle_orientation(
                &PointF64::new(0.0, 0.0),
                &PointF64::new(2.0, 2.0),
                &PointF64::new(0.0, 0.0)
            ),
            AngleOrientation::Colinear
        );

        assert_eq!(
            angle_orientation(
                &PointF64::new(0.0, 0.0),
                &PointF64::new(2.0, 2.0),
                &PointF64::new(4.0, 0.0),
            ),
            AngleOrientation::Clockwise
        );

        assert_eq!(
            angle_orientation(
                &PointF64::new(0.0, 0.0),
                &PointF64::new(4.0, 0.0),
                &PointF64::new(2.0, 2.0),
            ),
            AngleOrientation::CounterClockwise
        );

        assert_eq!(
            angle_orientation(
                &PointF64::new(4.0, 0.0),
                &PointF64::new(4.0, 0.0),
                &PointF64::new(2.0, 2.0),
            ),
            AngleOrientation::Colinear
        );
    }
}
