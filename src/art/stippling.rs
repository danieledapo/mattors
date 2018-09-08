//! Generate some stippling art.

extern crate image;
extern crate rand;

use art::random_point_in_bbox;
use drawing::Drawer;
use geo::{BoundingBox, PointU32};

/// The direction of gradient made of stippled points.
#[derive(Eq, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// The gradient is generated from left to right.
    LeftToRight,

    /// The gradient is generated from right to left.
    RightToLeft,

    /// The gradient is generated from top to bottom.
    TopToBottom,

    /// The gradient is generated from bottom to top.
    BottomToTop,
}

/// Stipple the given image in bands with increasing number of points to
/// simulate a gradient.
pub fn gradient<I>(
    img: &mut I,
    bands: u32,
    base_points_per_band: u32,
    grow_coeff: u32,
    pix: &I::Pixel,
    dir: Direction,
) where
    I: image::GenericImage,
    I::Pixel: ::std::fmt::Debug,
{
    let mut rng = rand::thread_rng();

    let mut drawer = Drawer::new_with_default_blending(img);

    let (width, height) = drawer.dimensions();
    let band_width = width / bands;
    let band_height = height / bands;

    let mut band = match dir {
        Direction::LeftToRight => BoundingBox::from_dimensions(band_width, height),
        Direction::RightToLeft => BoundingBox::from_dimensions_and_origin(
            &PointU32::new((bands - 1) * band_width, 0),
            band_width,
            height,
        ),
        Direction::TopToBottom => BoundingBox::from_dimensions(width, band_height),
        Direction::BottomToTop => BoundingBox::from_dimensions_and_origin(
            &PointU32::new(0, (bands - 1) * band_height),
            width,
            band_height,
        ),
    };

    let mut band_npoints = base_points_per_band;

    for i in 0..bands {
        for _ in 0..band_npoints {
            let point = random_point_in_bbox(&mut rng, &band);

            drawer.draw_pixel(point.x, point.y, &pix);
        }

        if i == bands - 1 {
            continue;
        }

        band_npoints += band_npoints * grow_coeff;

        let band_new_origin = match dir {
            Direction::LeftToRight => PointU32::new(band.max().x, 0),
            Direction::RightToLeft => PointU32::new(band.min().x - band_width, 0),
            Direction::TopToBottom => PointU32::new(0, band.max().y),
            Direction::BottomToTop => PointU32::new(0, band.min().y - band_height),
        };

        let (w, h) = band.dimensions().unwrap();
        band = BoundingBox::from_dimensions_and_origin(&band_new_origin, w, h);
    }
}
