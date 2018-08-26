//! Module that allows to compute the [Convex
//! Hull](https://en.wikipedia.org/wiki/Convex_hull) of a set of points.

extern crate num;

use std::cmp::Ordering;

use geo::{angle_orientation, polar_angle, AngleOrientation, Point};
use utils::cmp_f64;

/// Calculate the convex hull of a set of points and return the points that
/// compose the convex hull.
pub fn convex_hull<I>(points: I) -> Vec<Point<f64>>
where
    I: IntoIterator<Item = Point<f64>>,
{
    let mut points = points.into_iter().collect::<Vec<_>>();

    if points.len() < 2 {
        return points;
    }

    let lowest_point = *points
        .iter()
        .min_by(|p1, p2| {
            let ycmp = cmp_f64(p1.y, p2.y);

            if let Ordering::Equal = ycmp {
                cmp_f64(p1.x, p2.y)
            } else {
                ycmp
            }
        })
        .unwrap();

    // sort in descending order so that we remove points from the back which is
    // amortized O(1).
    points.sort_unstable_by(|p1, p2| {
        let a1 = polar_angle(&lowest_point, p1);
        let a2 = polar_angle(&lowest_point, p2);

        cmp_f64(a2, a1)
    });

    let mut hull = vec![];
    hull.push(points.pop().unwrap());
    hull.push(points.pop().unwrap());

    for point in points.into_iter().rev() {
        while hull.len() >= 2 {
            let orientation =
                angle_orientation(&hull[hull.len() - 2], hull.last().unwrap(), &point);

            match orientation {
                AngleOrientation::Clockwise | AngleOrientation::Colinear => hull.pop(),
                AngleOrientation::CounterClockwise => break,
            };
        }

        hull.push(point);
    }

    hull
}

#[cfg(test)]
mod tests {
    use super::convex_hull;

    use geo::Point;

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Point::new(392.0, 23.0),
            Point::new(134.0, 59.0),
            Point::new(251.0, 127.0),
            Point::new(266.0, 143.0),
            Point::new(380.0, 183.0),
            Point::new(337.0, 44.0),
            Point::new(229.0, 20.0),
            Point::new(378.0, 496.0),
            Point::new(392.0, 23.0),
        ];

        let hull = convex_hull(points);

        assert_eq!(
            hull,
            vec![
                Point::new(229.0, 20.0),
                Point::new(392.0, 23.0),
                Point::new(378.0, 496.0),
                Point::new(134.0, 59.0),
            ]
        );
    }
}
