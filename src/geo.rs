//! Handy `Point` struct and utility functions.

extern crate num;
extern crate rand;

use std::clone::Clone;
use std::convert::From;
use std::error::Error;
use std::str::FromStr;

/// Point specialized for `f64`
pub type PointF64 = Point<f64>;

/// Point specialized for `u32`
pub type PointU32 = Point<u32>;

/// Simple 2d Point struct
#[derive(Clone, Debug, PartialEq)]
pub struct Point<T> {
    /// x coordinate
    pub x: T,

    /// y coordinate
    pub y: T,
}

impl<T> Point<T> {
    /// create a new `Point` with the given `x` and `y` coordinates.
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: FromStr> FromStr for Point<T>
where
    T: Clone,
    T::Err: Error,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Result<Vec<T>, T::Err> = s.trim().split(',').map(|p| p.parse()).collect();

        match points {
            Err(e) => Err("bad coord number format, ".to_string() + e.description()),
            Ok(points) => {
                if points.len() != 2 {
                    Err("wrong number of coords, please pass x and y coords as floats separated by ','".to_string())
                } else {
                    Ok(Self {
                        x: points[0].clone(),
                        y: points[1].clone(),
                    })
                }
            }
        }
    }
}

/// Simple Triangle shape.
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle<P>
where
    P: num::Num + From<u8> + Clone,
{
    /// The points of the triangle
    pub points: [Point<P>; 3],
}

impl<P> Triangle<P>
where
    P: num::Num + From<u8> + Clone,
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
                (accx + pt.x.clone(), accy + pt.y.clone())
            });

        let avg_x = sum_x / From::from(3);
        let avg_y = sum_y / From::from(3);

        Point::new(avg_x, avg_y)
    }
}
