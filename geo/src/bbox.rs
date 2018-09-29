//! This module contains an implementation of a [Minimum bounding
//! box](https://en.wikipedia.org/wiki/Minimum_bounding_box) or AABB.

use std::iter::{Extend, FromIterator, IntoIterator};

use crate::point::Point;

/// Simple axis aligned bounding box implementation.
#[derive(Clone, Debug, PartialEq)]
pub struct BoundingBox<T> {
    min: Point<T>,
    max: Point<T>,
}

impl<T> BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    /// Create a new empty BoundingBox. An empty box does not contain any point
    /// and the min point is greater than the max one.
    pub fn new() -> Self {
        Self {
            min: Point::new(T::max_value(), T::max_value()),
            max: Point::new(T::min_value(), T::min_value()),
        }
    }

    /// Create a new BoundingBox that covers all the given points.
    pub fn from_points(pts: &[Point<T>]) -> Self {
        pts.iter().collect()
    }

    /// Create a new BoundingBox of the given width and height starting from the
    /// origin.
    pub fn from_dimensions(width: T, height: T) -> Self {
        Self::from_dimensions_and_origin(&Point::new(T::from(0), T::from(0)), width, height)
    }

    /// Create a new BoundingBox of the given width and height starting from the
    /// origin.
    pub fn from_dimensions_and_origin(origin: &Point<T>, width: T, height: T) -> Self {
        let mut bbox = Self::new();

        bbox.expand_by_point(origin);
        bbox.expand_by_point(&Point::new(origin.x + width, origin.y + height));

        bbox
    }

    /// Return whether this bounding box is empty.
    pub fn is_empty(&self) -> bool {
        self.max.x < self.min.x || self.max.y < self.min.y
    }

    /// Return the point with the lowest coordinates.
    pub fn min(&self) -> &Point<T> {
        &self.min
    }

    /// Return the point with the highest coordinates.
    pub fn max(&self) -> &Point<T> {
        &self.max
    }

    /// Return the area of this bounding box. None if the bounding box is empty.
    pub fn area(&self) -> Option<T> {
        self.dimensions().map(|(w, h)| w * h)
    }

    /// Return the width of this bounding box. None if the bounding box is empty.
    pub fn width(&self) -> Option<T> {
        self.dimensions().map(|(w, _)| w)
    }

    /// Return the height of this bounding box. None if the bounding box is empty.
    pub fn height(&self) -> Option<T> {
        self.dimensions().map(|(_, h)| h)
    }

    /// Return the width and the height of this bounding box as a tuple. None if
    /// the box is empty.
    pub fn dimensions(&self) -> Option<(T, T)> {
        if self.is_empty() {
            None
        } else {
            Some((self.max.x - self.min.x, self.max.y - self.min.y))
        }
    }

    /// Expand this bounding box by the given point.
    pub fn expand_by_point(&mut self, pt: &Point<T>) {
        self.min = self.min.lowest(pt);
        self.max = self.max.highest(pt);
    }

    /// Split this BoundingBox into 4 given a point inside this bounding box. If
    /// the bounding box is empty `None` is returned.
    pub fn split_at(&self, pt: &Point<T>) -> Option<(Self, Self, Self, Self)> {
        if !self.contains(pt) {
            return None;
        }

        Some((
            BoundingBox {
                min: self.min,
                max: *pt,
            },
            BoundingBox {
                min: Point::new(pt.x, self.min.y),
                max: Point::new(self.max.x, pt.y),
            },
            BoundingBox {
                min: Point::new(self.min.x, pt.y),
                max: Point::new(pt.x, self.max.y),
            },
            BoundingBox {
                min: *pt,
                max: self.max,
            },
        ))
    }

    /// Check if a point lies inside this bounding box.
    pub fn contains(&self, pt: &Point<T>) -> bool {
        self.min.x <= pt.x && self.max.x >= pt.x && self.min.y <= pt.y && self.max.y >= pt.y
    }

    /// Return the points of this rectangle in clockwise order.
    pub fn points(&self) -> [Point<T>; 4] {
        [
            self.min,
            Point::new(self.max.x, self.min.y),
            self.max,
            Point::new(self.min.x, self.max.y),
        ]
    }

    /// Return the center of this rectangle.
    pub fn center(&self) -> Point<T> {
        Point::new(
            (self.min.x + self.max.x) / T::from(2),
            (self.min.y + self.max.y) / T::from(2),
        )
    }
}

impl<T> Default for BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: 'a> FromIterator<&'a Point<T>> for BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    fn from_iter<I>(points: I) -> Self
    where
        I: IntoIterator<Item = &'a Point<T>>,
    {
        let mut bbox = BoundingBox::new();

        for point in points {
            bbox.expand_by_point(point);
        }

        bbox
    }
}

impl<'a, T: 'a> Extend<&'a Point<T>> for BoundingBox<T>
where
    T: num::Num + num::Bounded + From<u8> + Copy + PartialOrd,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a Point<T>>,
    {
        for pt in iter {
            self.expand_by_point(pt);
        }
    }
}

#[cfg(test)]
mod test {
    use super::BoundingBox;

    use geo::PointU32;

    #[test]
    fn test_contains() {
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(3, 5), 7, 5);

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
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(3, 5), 7, 5);

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
        let rec = BoundingBox::from_dimensions_and_origin(&PointU32::new(2, 4), 8, 6);

        assert_eq!(rec.center(), PointU32::new(6, 7));
    }

    #[test]
    fn test_dimensions() {
        let mut bbox = BoundingBox::new();
        assert!(bbox.is_empty());
        assert!(bbox.dimensions().is_none());

        bbox.expand_by_point(&PointU32::new(8, 8));
        assert_eq!(bbox.dimensions(), Some((0, 0)));
        assert_eq!(bbox.width(), Some(0));
        assert_eq!(bbox.height(), Some(0));

        bbox.expand_by_point(&PointU32::new(0, 0));
        assert_eq!(bbox.dimensions(), Some((8, 8)));
        assert_eq!(bbox.width(), Some(8));
        assert_eq!(bbox.height(), Some(8));
    }

    #[test]
    fn test_area() {
        let mut bbox = BoundingBox::new();

        assert!(bbox.area().is_none());

        bbox.expand_by_point(&PointU32::new(4, 4));
        assert_eq!(bbox.area(), Some(0));

        bbox.expand_by_point(&PointU32::new(8, 9));
        assert_eq!(bbox.area(), Some(20));
    }

    #[test]
    fn test_split_at() {
        let mut bbox = BoundingBox::new();

        assert!(bbox.split_at(&PointU32::new(0, 0)).is_none());

        bbox.expand_by_point(&PointU32::new(0, 0));
        assert!(bbox.split_at(&PointU32::new(42, 42)).is_none());
        assert_eq!(
            bbox.split_at(&PointU32::new(0, 0)),
            Some((
                BoundingBox {
                    min: PointU32::new(0, 0),
                    max: PointU32::new(0, 0)
                },
                BoundingBox {
                    min: PointU32::new(0, 0),
                    max: PointU32::new(0, 0)
                },
                BoundingBox {
                    min: PointU32::new(0, 0),
                    max: PointU32::new(0, 0)
                },
                BoundingBox {
                    min: PointU32::new(0, 0),
                    max: PointU32::new(0, 0)
                }
            ))
        );

        bbox.expand_by_point(&PointU32::new(8, 8));
        assert!(bbox.split_at(&PointU32::new(42, 42)).is_none());
        assert_eq!(
            bbox.split_at(&PointU32::new(4, 4)),
            Some((
                BoundingBox {
                    min: PointU32::new(0, 0),
                    max: PointU32::new(4, 4)
                },
                BoundingBox {
                    min: PointU32::new(4, 0),
                    max: PointU32::new(8, 4)
                },
                BoundingBox {
                    min: PointU32::new(0, 4),
                    max: PointU32::new(4, 8)
                },
                BoundingBox {
                    min: PointU32::new(4, 4),
                    max: PointU32::new(8, 8)
                }
            ))
        );
    }
}
