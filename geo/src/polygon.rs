//! This module contains a simple definition of a closed polygon.

extern crate num;

use std::cmp::Ordering;

use bbox::BoundingBox;
use line::LineEquation;
use point::Point;
use utils::cmp_floats;

/// A simple closed Polygon primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon<T> {
    points: Vec<Point<T>>,
    bbox: BoundingBox<T>,
}

impl<T> Polygon<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    /// Create a new polygon from the given set of points. Returns None if the
    /// points are too few to form a polygon. Note that if the last point
    /// doesn't match the first one then the first one is automatically appended
    /// at the back.
    pub fn new<I: IntoIterator<Item = Point<T>>>(points: I) -> Option<Self> {
        let points = points.into_iter();

        let mut polygon_points = if let (_, Some(c)) = points.size_hint() {
            Vec::with_capacity(c)
        } else {
            vec![]
        };
        let mut bbox = BoundingBox::new();

        for point in points {
            bbox.expand_by_point(&point);
            polygon_points.push(point);
        }

        if polygon_points.len() < 3 {
            None
        } else {
            if polygon_points[0] != polygon_points[polygon_points.len() - 1] {
                let p = polygon_points[0];
                polygon_points.push(p);
            }

            Some(Polygon {
                points: polygon_points,
                bbox,
            })
        }
    }

    /// Return the minimum bounding box for this polygon.
    pub fn bounding_box(&self) -> &BoundingBox<T> {
        &self.bbox
    }

    /// Return an iterator over the vertices of the polygon.
    pub fn points(&self) -> &[Point<T>] {
        &self.points
    }

    /// Return all the edges as tuples of the polygon.
    pub fn edges(&self) -> impl Iterator<Item = (&Point<T>, &Point<T>)> {
        self.points.windows(2).map(|e| (&e[0], &e[1]))
    }

    /// Return whether the given point is contained in this polygon using the
    /// [1][Ray casting algorithm].
    ///
    /// [1]: (https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm).
    #[cfg_attr(feature = "cargo-clippy", allow(collapsible_if))]
    pub fn contains(&self, pt: &Point<T>) -> bool
    where
        T: num::Float + From<f32>,
    {
        // the bounding box check isn't accurate, but it's fast and  if the
        // point is not in the box then it's definitely not in the polygon
        // either.
        if !self.bbox.contains(pt) {
            return false;
        }

        let mut inside = false;

        for (p0, p1) in self.edges() {
            if in_range(&p0.y, &p1.y, &pt.y) {
                let line = LineEquation::between(p0, p1);

                if let Some(x) = line.x_at(pt.y) {
                    // if the x matches then the point is on the boundary
                    if let Ordering::Equal = cmp_floats(x, pt.x) {
                        return true;
                    }

                    // we need to exclude edges where the point.y equals the
                    // edge max y, because otherwise we'd check the same vertex
                    // twice which would lead inside to not change(which is bad).
                    if (p0.y <= pt.y && p1.y == pt.y) || (p1.y <= pt.y && p0.y == pt.y) {
                        continue;
                    }

                    if x < pt.x {
                        inside = !inside;
                    }
                } else {
                    // if an edge is horizontal then check if pt.x lies in its
                    // range.

                    if in_range(&p0.x, &p1.x, &pt.x) {
                        return true;
                    }
                }
            }
        }

        inside
    }
}

fn in_range<T: PartialOrd>(a: &T, b: &T, v: &T) -> bool {
    let (min, max) = if a < b { (a, b) } else { (b, a) };

    min <= v && max >= v
}

#[cfg(test)]
mod tests {
    use super::Polygon;

    use geo::{BoundingBox, PointF64, PointU32};

    #[test]
    fn test_polygon_new() {
        assert_eq!(Polygon::<u32>::new(Vec::new()), None);
        assert_eq!(Polygon::new(vec![PointU32::new(0, 0)]), None);
        assert_eq!(
            Polygon::new(vec![PointU32::new(0, 0), PointU32::new(3, 2)]),
            None
        );

        assert_eq!(
            Polygon::new(vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(1, 1),
            ]),
            Some(Polygon {
                points: vec![
                    PointU32::new(0, 0),
                    PointU32::new(8, 6),
                    PointU32::new(1, 1),
                    PointU32::new(0, 0),
                ],
                bbox: BoundingBox::from_dimensions(8, 6),
            })
        );

        assert_eq!(
            Polygon::new(vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
            ]),
            Some(Polygon {
                points: vec![
                    PointU32::new(0, 0),
                    PointU32::new(8, 6),
                    PointU32::new(3, 2),
                    PointU32::new(1, 1),
                    PointU32::new(0, 0),
                ],
                bbox: BoundingBox::from_dimensions(8, 6),
            })
        );
    }

    #[test]
    fn test_polygon_points() {
        let manifest = [
            vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
            ],
            vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
                PointU32::new(0, 0),
            ],
        ];

        for points in &manifest {
            let polygon = Polygon::new(points.clone());
            assert!(polygon.is_some());

            let polygon = polygon.unwrap();

            assert_eq!(
                polygon.points(),
                [
                    PointU32::new(0, 0),
                    PointU32::new(8, 6),
                    PointU32::new(3, 2),
                    PointU32::new(1, 1),
                    PointU32::new(0, 0),
                ]
            );
        }
    }

    #[test]
    fn test_polygon_edges() {
        let manifest = [
            vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
            ],
            vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
                PointU32::new(0, 0),
            ],
        ];

        for points in &manifest {
            let polygon = Polygon::new(points.clone());
            assert!(polygon.is_some());

            let polygon = polygon.unwrap();

            assert_eq!(
                polygon.edges().collect::<Vec<_>>(),
                vec![
                    (&PointU32::new(0, 0), &PointU32::new(8, 6)),
                    (&PointU32::new(8, 6), &PointU32::new(3, 2)),
                    (&PointU32::new(3, 2), &PointU32::new(1, 1)),
                    (&PointU32::new(1, 1), &PointU32::new(0, 0)),
                ]
            );
        }
    }

    #[test]
    fn test_polygon_contains_points() {
        let points = vec![
            PointF64::new(0.0, 0.0),
            PointF64::new(13.0, 6.0),
            PointF64::new(3.0, 4.0),
            PointF64::new(2.0, 2.0),
        ];

        let polygon = Polygon::new(points.clone()).unwrap();

        // must contain all the vertices
        for pt in points {
            assert!(polygon.contains(&pt));
        }

        // some points on the edges
        assert!(polygon.contains(&PointF64::new(5.0, 3.75)));
        assert!(polygon.contains(&PointF64::new(1.0, 1.0)));

        // some points in the inside
        assert!(polygon.contains(&PointF64::new(3.0, 3.0)));
        assert!(polygon.contains(&PointF64::new(4.0, 4.0)));

        // outside points
        assert!(!polygon.contains(&PointF64::new(0.0, 1.0)));
        assert!(!polygon.contains(&PointF64::new(1.5, 9.0)));
        assert!(!polygon.contains(&PointF64::new(1.5, 9.0)));
    }

    #[test]
    fn test_polygon_square_triangle_contains_boundary() {
        let points = vec![
            PointF64::new(39.0, 219.0),
            PointF64::new(83.0, 1.0),
            PointF64::new(0.0, 219.0),
        ];

        let poly = Polygon::new(points.clone()).unwrap();

        for pt in points {
            assert!(poly.contains(&pt), "point {:?} should be contained", pt);
        }
    }

    #[test]
    fn test_polygon_square_triangle_contains() {
        let poly = Polygon {
            points: vec![
                PointF64::new(0.0, 0.0),
                PointF64::new(2.0, 247.0),
                PointF64::new(0.0, 247.0),
                PointF64::new(0.0, 0.0),
            ],
            bbox: BoundingBox::from_dimensions(2.0, 247.0),
        };

        assert!(poly.contains(&PointF64::new(1.0, 247.0)));
    }
}
