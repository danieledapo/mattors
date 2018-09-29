//! Module that allows to compute the [Convex
//! Hull](https://en.wikipedia.org/wiki/Convex_hull) of a set of points.

extern crate num;

use std::cmp::Ordering;

use angle::{angle_orientation, polar_angle, AngleOrientation};
use point::Point;
use utils::cmp_floats;

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
            let ycmp = cmp_floats(p1.y, p2.y);

            if let Ordering::Equal = ycmp {
                cmp_floats(p1.x, p2.x)
            } else {
                ycmp
            }
        }).unwrap();

    // sort in descending order so that we remove points from the back which is
    // amortized O(1).
    points.sort_unstable_by(|p1, p2| {
        let a1 = polar_angle(&lowest_point, p1);
        let a2 = polar_angle(&lowest_point, p2);

        let angle_cmp = cmp_floats(a2, a1);

        if let Ordering::Equal = angle_cmp {
            let ycmp = cmp_floats(p2.y, p1.y);

            if let Ordering::Equal = ycmp {
                cmp_floats(p2.x, p1.x)
            } else {
                ycmp
            }
        } else {
            angle_cmp
        }
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

    extern crate proptest;

    use proptest::prelude::*;

    use geo::{Point, Polygon};

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

    #[test]
    fn test_convex_hull_multiple_points_on_same_y() {
        let points = vec![
            Point::new(4.0, 40.0),
            Point::new(21.0, 21.0),
            Point::new(37.0, 32.0),
            Point::new(40.0, 21.0),
        ];

        let hull = convex_hull(points);

        assert_eq!(
            hull,
            vec![
                Point::new(21.0, 21.0),
                Point::new(40.0, 21.0),
                Point::new(37.0, 32.0),
                Point::new(4.0, 40.0),
            ]
        );
    }

    #[test]
    fn test_convex_hull_colinear() {
        let points = vec![
            Point::new(12.0, 41.0),
            Point::new(17.0, 36.0),
            Point::new(42.0, 11.0),
            Point::new(0.0, 12.0),
        ];

        let hull = convex_hull(points);

        assert_eq!(
            hull,
            vec![
                Point::new(42.0, 11.0),
                Point::new(12.0, 41.0),
                Point::new(0.0, 12.0),
            ]
        );
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(500))]
        #[test]
        fn prop_convex_contains_all_the_points(
            points in prop::collection::hash_set((0_u8..255, 0_u8..255), 3..100)
        ) {
            let points = points
                .into_iter()
                .map(|(x, y)| Point::new(f64::from(x), f64::from(y)))
                .collect::<Vec<_>>();

            let hull = convex_hull(points.clone());
            prop_assume!(hull.len() > 2);

            let hull = Polygon::new(hull).unwrap();

            for pt in &points {
                assert!(
                    hull.contains(pt),
                    "points {:?} hull {:?} point {:?}",
                    points,
                    hull,
                    pt
                );
            }
        }
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config::with_cases(100))]
        #[test]
        fn prop_convex_hull_lies_on_boundary(
            points in prop::collection::vec((0_u8..255, 0_u8..255), 1..100)
        ) {
            _prop_convex_hull_lies_on_boundary(points)
        }
    }

    fn _prop_convex_hull_lies_on_boundary(points: Vec<(u8, u8)>) {
        let points = points
            .into_iter()
            .map(|(x, y)| Point::new(x, y))
            .collect::<Vec<_>>();

        let hull = convex_hull(points.iter().map(|p| p.cast::<f64>()));

        for pt in hull {
            let pt = Point::new(pt.x as u8, pt.y as u8);

            let on_boundary = points.iter().find(|h| **h == pt).is_some();

            assert!(on_boundary);
        }
    }
}
