//! Geometric functions.

pub mod delaunay;
pub mod line;
pub mod point;
pub mod rect;
pub mod triangle;

pub use self::line::LineEquation;
pub use self::point::{Point, PointF64, PointI32, PointU32};
pub use self::rect::Rect;
pub use self::triangle::Triangle;
