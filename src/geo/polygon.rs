//! This module contains a simple definition of a closed polygon.

use geo::Point;

/// A simple closed Polygon primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct Polygon<T>(Vec<Point<T>>);

impl<T> Polygon<T> {
    /// Create a new polygon from the given set of points. Returns None if the
    /// points are too few to form a polygon. Note that if the last point
    /// doesn't match the first one then the first one is automatically appended
    /// at the back.
    pub fn new<I: IntoIterator<Item = Point<T>>>(points: I) -> Option<Self>
    where
        T: PartialEq + Copy,
    {
        let mut points = points.into_iter().collect::<Vec<_>>();

        if points.len() < 3 {
            None
        } else {
            if points[0] != points[points.len() - 1] {
                let p = points[0];
                points.push(p);
            }

            Some(Polygon(points))
        }
    }

    /// Return an iterator over the vertices of the polygon.
    pub fn points(&self) -> &[Point<T>] {
        &self.0
    }

    /// Return all the edges as tuples of the polygon.
    pub fn edges(&self) -> impl Iterator<Item = (&Point<T>, &Point<T>)> {
        self.0.windows(2).map(|e| (&e[0], &e[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::Polygon;

    use geo::PointU32;

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
            Some(Polygon(vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(1, 1),
                PointU32::new(0, 0),
            ]))
        );

        assert_eq!(
            Polygon::new(vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
            ]),
            Some(Polygon(vec![
                PointU32::new(0, 0),
                PointU32::new(8, 6),
                PointU32::new(3, 2),
                PointU32::new(1, 1),
                PointU32::new(0, 0),
            ]))
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
}
