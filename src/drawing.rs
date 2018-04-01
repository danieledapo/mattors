// most of this stuff is already implemented in the imageprocs crate, but the
// best way to learn is by reimplementing, so...

extern crate image;

use std::iter::Iterator;
use std::mem;

use point::{Point, PointU32};

/// Iterator that returns all the points that compose the line from start to
/// end. It uses the [Bresenham's line
/// algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm) to
/// interpolate the points in the line.
#[derive(Debug)]
pub struct BresenhamLineIter {
    // this struct is designed to work for non steep lines. In case we actually
    // want to iterate over a steep line then the `new` function swaps x with y,
    // sets `is_steep` that is then checked in `next` and swaps x with y again
    // if the flag is set. It also assumes that `start` is the more "bottom
    // left" than `end`(this invariant is also ensured by `new`).
    start: Point<i64>,
    end: PointU32,
    is_steep: bool,
    d: i64,
    dx: i64,
    dy: i64,
}

impl BresenhamLineIter {
    /// Creates a new `BresenhamLineIter` iterator to return all points between
    /// `start` and `end` both included.
    pub fn new(mut start: PointU32, mut end: PointU32) -> BresenhamLineIter {
        let mut dx = (i64::from(end.x) - i64::from(start.x)).abs();
        let mut dy = (i64::from(end.y) - i64::from(start.y)).abs();

        let mut is_steep = false;

        // find out whether the line is steep that is that whether it grows faster
        // in y or in x and call the appropriate implementation. The algorithms are
        // the mirrors of each other, but the main idea is the same: the bump of the
        // slowest coordinate is governed by whether the value is closer to the new
        // coord or not.
        if dx >= dy {
            if start.x > end.x {
                mem::swap(&mut start, &mut end);
            }
        } else {
            if start.y > end.y {
                mem::swap(&mut start, &mut end);
            }

            is_steep = true;
            mem::swap(&mut start.x, &mut start.y);
            mem::swap(&mut end.x, &mut end.y);
            mem::swap(&mut dx, &mut dy);
        }

        let start = Point {
            x: i64::from(start.x),
            y: i64::from(start.y),
        };

        BresenhamLineIter {
            start,
            end,
            is_steep,
            dx,
            dy,
            d: 2 * dy - dx,
        }
    }

    // calculate next non steep point in the line
    fn next_non_steep_point(&mut self) -> Option<PointU32> {
        if self.start.x > i64::from(self.end.x) {
            return None;
        }

        if self.start.x < 0 || self.start.y < 0 {
            return None;
        }

        let old = PointU32 {
            x: self.start.x as u32,
            y: self.start.y as u32,
        };

        if self.d > 0 {
            self.start.y += 1;
            self.d -= 2 * self.dx;
        }

        self.d += 2 * self.dy;

        self.start.x += 1;

        Some(old)
    }
}

impl Iterator for BresenhamLineIter {
    type Item = PointU32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_non_steep_point().map(|mut res| {
            // in case the line is steep then we need to swap back the
            // coordinates before returning to reverse the swap done in `new`.
            if self.is_steep {
                mem::swap(&mut res.x, &mut res.y);
            }
            res
        })
    }
}

/// Draw a line on the given image using [Bresenham's line
/// algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
pub fn line<I>(img: &mut I, start: PointU32, end: PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
{
    let it = BresenhamLineIter::new(start, end);
    for pt in it {
        if pt.x >= img.width() || pt.y >= img.height() {
            break;
        }

        img.put_pixel(pt.x, pt.y, *pix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bresenham_line() {
        let origin = Point { x: 0, y: 0 };

        assert_eq!(
            BresenhamLineIter::new(origin.clone(), origin.clone()).collect::<Vec<_>>(),
            vec![origin.clone()],
            "line from origin to origin"
        );

        let bis = Point { x: 3, y: 3 };
        let bis_exp_points = vec![
            origin.clone(),
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            bis.clone(),
        ];

        assert_eq!(
            BresenhamLineIter::new(origin.clone(), bis.clone()).collect::<Vec<_>>(),
            bis_exp_points,
            "line from origin to bisec {:?}",
            bis
        );

        assert_eq!(
            BresenhamLineIter::new(bis.clone(), origin.clone()).collect::<Vec<_>>(),
            bis_exp_points,
            "line from bisec {:?} to origin",
            bis
        );
    }

    #[test]
    fn test_bresenham_line_non_steep() {
        let origin = Point { x: 0, y: 0 };
        let non_steep_pt = Point { x: 3, y: 1 };
        let exp_points = vec![
            origin.clone(),
            Point { x: 1, y: 0 },
            Point { x: 2, y: 1 },
            non_steep_pt.clone(),
        ];

        assert_eq!(
            BresenhamLineIter::new(origin.clone(), non_steep_pt.clone()).collect::<Vec<_>>(),
            exp_points,
            "line from origin to non steep {:?}",
            non_steep_pt
        );

        assert_eq!(
            BresenhamLineIter::new(non_steep_pt.clone(), origin.clone()).collect::<Vec<_>>(),
            exp_points,
            "line from non steep {:?} to origin",
            non_steep_pt
        );
    }

    #[test]
    fn test_bresenham_line_steep() {
        let origin = Point { x: 0, y: 0 };
        let steep_pt = Point { x: 1, y: 3 };
        let exp_points = vec![
            origin.clone(),
            Point { x: 0, y: 1 },
            Point { x: 1, y: 2 },
            steep_pt.clone(),
        ];

        assert_eq!(
            BresenhamLineIter::new(origin.clone(), steep_pt.clone()).collect::<Vec<_>>(),
            exp_points,
            "line from origin to steep {:?}",
            steep_pt
        );

        assert_eq!(
            BresenhamLineIter::new(steep_pt.clone(), origin.clone()).collect::<Vec<_>>(),
            exp_points,
            "line from steep {:?} to origin",
            steep_pt
        );
    }
}
