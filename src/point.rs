use std::clone::Clone;
use std::error::Error;
use std::str::FromStr;

pub type PointF64 = Point<f64>;
pub type PointU32 = Point<u32>;

/// Simple 2d Point struct
#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
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
