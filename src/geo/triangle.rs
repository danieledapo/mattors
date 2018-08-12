//! Module to work with triangles.

extern crate num;

use geo::{LineEquation, Point};

/// Simple Triangle shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle<P> {
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
    pub fn circumcenter(&self) -> Option<Point<P>>
    where
        P: ::std::fmt::Debug,
    {
        let p0p1 = LineEquation::between(&self.points[0], &self.points[1]);
        let p0p2 = LineEquation::between(&self.points[0], &self.points[2]);

        let mid_p0p1 = self.points[0].midpoint(&self.points[1]);
        let mid_p0p2 = self.points[0].midpoint(&self.points[2]);

        let bisec_p0p1 = p0p1.perpendicular(&mid_p0p1);
        let bisec_p0p2 = p0p2.perpendicular(&mid_p0p2);

        let res = bisec_p0p1.intersection(&bisec_p0p2);

        if res.is_none() {
            println!(
                "p0p1 {:?} p0p2 {:?} bisec_p0p1 {:?} bisec_p0p2 {:?}",
                p0p1, p0p2, bisec_p0p1, bisec_p0p2
            );
        }

        res
    }

    /// Return the circumcicle that encloses this triangle as a pair of
    /// circumcenter and radius _squared_.
    pub fn squared_circumcircle<O>(&self) -> Option<(Point<P>, O)>
    where
        O: num::Num + From<P> + Copy,
        P: ::std::fmt::Debug,
    {
        self.circumcenter().map(|circumcenter| {
            let squared_radius = circumcenter.squared_dist(&self.points[0]);

            (circumcenter, squared_radius)
        })
    }
}

impl<P> Triangle<P>
where
    P: num::Num + num::Signed + From<u8> + Copy + PartialOrd,
    f64: From<P>,
{
    /// Return the circumcicle that encloses this triangle as a pair of
    /// circumcenter and radius.
    pub fn circumcircle(&self) -> Option<(Point<P>, f64)>
    where
        P: ::std::fmt::Debug,
    {
        self.circumcenter().map(|circumcenter| {
            let radius = circumcenter.dist(&self.points[0]);

            (circumcenter, radius)
        })
    }
}

#[cfg(test)]
mod test {
    use super::Triangle;
    use geo::PointI32;

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
}
