//! Low level implementation details of the triangle primitive.

extern crate geo;

use self::geo::PointU32;

use drawing::line::BresenhamLineIter;

/// Iterator that returns the edge points of a flat triangle that is a triangle
/// that has at least 2 points on the same y.
pub struct FlatTriangleIter {
    last_start: PointU32,
    last_end: PointU32,
    p1p2_iter: BresenhamLineIter,
    p1p3_iter: BresenhamLineIter,
    over: bool,
}

impl FlatTriangleIter {
    /// Create a new `FlatTriangleIter`.
    /// invariant: `p2` and `p3` are the points on the flat line.
    pub fn new(p1: PointU32, p2: PointU32, p3: PointU32) -> FlatTriangleIter {
        FlatTriangleIter {
            last_start: p1,
            last_end: p1,
            p1p2_iter: BresenhamLineIter::new(p1, p2),
            p1p3_iter: BresenhamLineIter::new(p1, p3),
            over: false,
        }
    }
}

impl Iterator for FlatTriangleIter {
    type Item = (PointU32, PointU32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.over {
            return None;
        }

        let res = (self.last_start, self.last_end);

        // advance the current points, but make sure the y coord actually
        // changes because otherwise we could potentially draw a line on the
        // same y coordinates multiple times.
        loop {
            match self.p1p2_iter.next() {
                Some(new_start) => {
                    if new_start.y != self.last_start.y {
                        self.last_start = new_start;
                        break;
                    }
                }
                None => {
                    self.over = true;
                    break;
                }
            }
        }

        loop {
            match self.p1p3_iter.next() {
                Some(new_end) => {
                    if new_end.y != self.last_end.y {
                        self.last_end = new_end;
                        break;
                    }
                }
                None => {
                    self.over = true;
                    break;
                }
            }
        }

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use self::geo::Point;
    use super::*;

    #[test]
    fn test_flat_upper_triangle_iter() {
        let p1 = Point::new(4, 0);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(8, 2);

        let exp_points = vec![
            (p1, p1),
            (PointU32::new(3, 1), PointU32::new(6, 1)),
            (p2, p3),
        ];

        assert_eq!(
            FlatTriangleIter::new(p1, p2, p3).collect::<Vec<_>>(),
            exp_points
        );
    }

    #[test]
    fn test_flat_bottom_triangle_iter() {
        let p1 = Point::new(2, 0);
        let p2 = Point::new(6, 0);
        let p3 = Point::new(4, 2);

        let exp_points = vec![
            (p3, p3),
            (PointU32::new(3, 1), PointU32::new(5, 1)),
            (p1, p2),
        ];

        assert_eq!(
            FlatTriangleIter::new(p3, p1, p2).collect::<Vec<_>>(),
            exp_points
        );
    }
}
