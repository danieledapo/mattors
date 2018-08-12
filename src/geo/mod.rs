//! Geometric functions.

pub mod line;
pub mod point;
pub mod triangle;

pub use self::line::LineEquation;
pub use self::point::{Point, PointF64, PointI32, PointU32};
pub use self::triangle::Triangle;
