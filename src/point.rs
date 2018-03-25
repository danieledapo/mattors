use std::error::Error;
use std::num::ParseFloatError;
use std::str::FromStr;

/// Simple 2d Point struct
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Result<Vec<f64>, ParseFloatError> =
            s.trim().split(',').map(|p| p.parse()).collect();

        match points {
            Err(e) => Err("bad coord number format, ".to_string() + e.description()),
            Ok(points) => {
                if points.len() != 2 {
                    Err("wrong number of coords, please pass x and y coords as floats separated by ','".to_string())
                } else {
                    Ok(Self {
                        x: points[0],
                        y: points[1],
                    })
                }
            }
        }
    }
}
