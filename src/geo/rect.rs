//! This module contains functions to work with rectangles.

extern crate num;

use geo::Point;

/// Simple struct representing a rectangle.
#[derive(Clone, Debug, PartialEq)]
pub struct Rect<T> {
    /// The origin point of this rectangle. If this rectangle is used for screen
    /// coordinates then it should be the top left point, whereas for algebra
    /// the bottom left one.
    pub origin: Point<T>,

    /// The width of the rectangle.
    pub width: T,

    /// The height of the rectangle.
    pub height: T,
}

impl<T> Rect<T>
where
    T: num::Num + From<u8> + Copy + PartialOrd,
{
    /// Create a new Rectangle with the given parameters.
    pub fn new(origin: Point<T>, width: T, height: T) -> Self {
        Self {
            origin,
            width,
            height,
        }
    }

    /// Check if a point lies inside this rectangle. Doesn't work if either
    /// width or height are negative.
    pub fn contains(&self, pt: &Point<T>) -> bool {
        self.origin.x <= pt.x
            && self.origin.x + self.width >= pt.x
            && self.origin.y <= pt.y
            && self.origin.y + self.height >= pt.y
    }

    /// Return the points of this rectangle in clockwise order.
    pub fn points(&self) -> [Point<T>; 4] {
        [
            self.origin.clone(),
            Point::new(self.origin.x + self.width, self.origin.y),
            Point::new(self.origin.x + self.width, self.origin.y + self.height),
            Point::new(self.origin.x, self.origin.y + self.height),
        ]
    }

    /// Return the center of this rectangle.
    pub fn center(&self) -> Point<T> {
        Point::new(
            (self.origin.x + self.width) / T::from(2),
            (self.origin.y + self.height) / T::from(2),
        )
    }
}

#[cfg(test)]
mod test {
    use super::Rect;

    use geo::PointU32;

    #[test]
    fn test_contains() {
        let rec = Rect::new(PointU32::new(3, 5), 7, 5);

        assert_eq!(rec.contains(&PointU32::new(0, 0)), false);
        assert_eq!(rec.contains(&PointU32::new(4, 0)), false);
        assert_eq!(rec.contains(&PointU32::new(0, 8)), false);
        assert_eq!(rec.contains(&PointU32::new(40, 40)), false);

        assert_eq!(rec.contains(&PointU32::new(3, 5)), true);
        assert_eq!(rec.contains(&PointU32::new(5, 7)), true);
        assert_eq!(rec.contains(&PointU32::new(10, 10)), true);
    }

    #[test]
    fn test_points() {
        let rec = Rect::new(PointU32::new(3, 5), 7, 5);

        assert_eq!(
            rec.points(),
            [
                PointU32::new(3, 5),
                PointU32::new(10, 5),
                PointU32::new(10, 10),
                PointU32::new(3, 10),
            ]
        )
    }

    #[test]
    fn test_center() {
        let rec = Rect::new(PointU32::new(2, 4), 8, 6);

        assert_eq!(rec.center(), PointU32::new(5, 5));
    }
}
