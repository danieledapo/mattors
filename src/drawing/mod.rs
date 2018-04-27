//! Simple module to draw basic shapes on an image. Most of this stuff is
//! already implemented in the imageprocs crate, but the best way to learn is by
//! reimplementing, so...

pub mod line;
pub mod triangle;

extern crate image;

use std::fmt::Debug;

use self::image::Pixel;

use self::line::BresenhamLineIter;
use self::triangle::FlatTriangleIter;
use geo::{Point, PointU32};

/// Draw a line on the given image using [Bresenham's line
/// algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
pub fn line<I>(img: &mut I, start: PointU32, end: PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
    I::Pixel: Debug,
{
    let it = BresenhamLineIter::new(start, end);
    for pt in it {
        if pt.x >= img.width() || pt.y >= img.height() {
            break;
        }

        let mut new_pix = img.get_pixel_mut(pt.x, pt.y);
        new_pix.blend(pix);

        // TODO: make blending optional
        // img.put_pixel(pt.x, pt.y, *pix);
    }
}

/// Draw a hollow triangle on the given image.
pub fn hollow_triangle<I>(img: &mut I, p1: &PointU32, p2: &PointU32, p3: &PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
    I::Pixel: Debug,
{
    line(img, p1.clone(), p2.clone(), pix);
    line(img, p1.clone(), p3.clone(), pix);
    line(img, p2.clone(), p3.clone(), pix);
}

/// Draw a triangle on the given image filled with the given `pix`.
pub fn triangle<I>(img: &mut I, p1: &PointU32, p2: &PointU32, p3: &PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
    I::Pixel: Debug,
{
    // the idea here is pretty simple: divide the triangle in an upper and
    // bottom flat triangles. At that point draw horizontal lines between the
    // edge points of the triangle.
    //
    //          /\
    // _______ /__\_____________  separating line
    //         \   \
    //           \  \
    //             \ \
    //               \

    let (tl, mid, br) = {
        // ugly as hell, but easier than hand written comparisons...
        let mut tmp = [p1, p2, p3];
        tmp.sort_by_key(|p| (p.y, p.x));

        (tmp[0], tmp[1], tmp[2])
    };

    let mid_y = f64::from(mid.y);
    let tl_y = f64::from(tl.y);
    let br_y = f64::from(br.y);
    let br_x = f64::from(br.x);
    let tl_x = f64::from(tl.x);

    let break_point = Point::new(
        (tl_x + (mid_y - tl_y) / (br_y - tl_y) * (br_x - tl_x)) as u32,
        mid.y,
    );

    let upper_triangle = FlatTriangleIter::new(tl, mid, &break_point);
    for (start, end) in upper_triangle {
        line(img, start, end, pix);
    }

    let mut bottom_triangle = FlatTriangleIter::new(br, &break_point, mid).peekable();
    loop {
        let mpoints = bottom_triangle.next();

        match mpoints {
            Some((start, end)) => {
                // make sure to do not draw the line between the last points because
                // it's the line that separates the upper_triangle and bottom_triangle
                // and we've already drawn it in the upper_triangle loop. This is
                // because we don't want to blend the pixels twice.
                let are_last_points = bottom_triangle.peek().is_none();

                if !are_last_points {
                    line(img, start, end, pix);
                }
            }
            _ => break,
        }
    }
}
