extern crate image;

use point::PointU32;

/// Draw a line on the given image using [Bresenham's line
/// algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm).
pub fn line<I>(img: &mut I, start: &PointU32, end: &PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
{
    let dx = i64::from(end.x) - i64::from(start.x);
    let dy = i64::from(end.y) - i64::from(start.y);

    // find out whether the line is steep that is that whether it grows faster
    // in y or in x and call the appropriate implementation. The algorithms are
    // the mirrors of each other, but the main idea is the same: the bump of the
    // slowest coordinate is governed by whether the value is closer to the new
    // coord or not.
    if dy.abs() < dx.abs() {
        let (start, end) = if start.x > end.x {
            (end, start)
        } else {
            (start, end)
        };

        line_non_steep(img, start, end, pix);
    } else {
        let (start, end) = if start.y > end.y {
            (end, start)
        } else {
            (start, end)
        };

        line_steep(img, start, end, pix);
    }
}

fn line_non_steep<I>(img: &mut I, start: &PointU32, end: &PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
{
    let dx = i64::from(end.x) - i64::from(start.x);

    let (dy, ystep) = {
        let d = i64::from(end.y) - i64::from(start.y);
        (d.abs(), d.signum())
    };

    let mut d = 2 * dy - dx;
    let mut y = i64::from(start.y);

    for x in (start.x)..(end.x + 1) {
        if x < img.width() && y >= 0 && y < i64::from(img.height()) {
            img.put_pixel(x, y as u32, *pix);
        }

        if d > 0 {
            y += ystep;
            d -= 2 * dx;
        }

        d += 2 * dy;
    }
}

fn line_steep<I>(img: &mut I, start: &PointU32, end: &PointU32, pix: &I::Pixel)
where
    I: image::GenericImage,
{
    let dy = i64::from(end.y) - i64::from(start.y);

    let (dx, xstep) = {
        let d = i64::from(end.x) - i64::from(start.x);
        (d.abs(), d.signum())
    };

    let mut d = 2 * dx - dy;
    let mut x = i64::from(start.x);

    for y in (start.y)..(end.y + 1) {
        if x >= 0 && x < i64::from(img.width()) && y < img.height() {
            img.put_pixel(x as u32, y, *pix);
        }

        if d > 0 {
            x += xstep;
            d -= 2 * dy;
        }

        d += 2 * dx;
    }
}
