//! Generate some stippling art.

extern crate image;
extern crate rand;

use art::random_point_in_bbox;
use drawing::Drawer;
use geo::{BoundingBox, PointU32};

/// The direction of gradient made of stippled points.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
/// simulate a gradient. Inspired by http://www.tylerlhobbs.com/works/series/st.
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

    let mut drawer = Drawer::new_with_no_blending(img);

    let (width, height) = drawer.dimensions();

    let mut band = initial_band(dir, width, height, bands);
    let mut band_npoints = base_points_per_band;

    for i in 0..bands {
        for _ in 0..band_npoints {
            let point = random_point_in_bbox(&mut rng, &band);

            drawer.draw_pixel(point.x, point.y, &pix);
        }

        // prevent overflow when dir is either RightToLeft or BottomToTop,
        // because at the (bands - 1)-th iteration we reached x = 0 or y = 0 and
        // we cannot advance anymore.
        if i == bands - 1 {
            continue;
        }

        band = advance_band(&band, dir);
        band_npoints += band_npoints * grow_coeff;
    }
}

fn initial_band(dir: Direction, width: u32, height: u32, bands: u32) -> BoundingBox<u32> {
    let band_width = width / bands;
    let band_height = height / bands;

    match dir {
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
    }
}

fn advance_band(band: &BoundingBox<u32>, dir: Direction) -> BoundingBox<u32> {
    let (band_width, band_height) = band.dimensions().unwrap();

    let band_new_origin = match dir {
        Direction::LeftToRight => PointU32::new(band.max().x, 0),
        Direction::RightToLeft => PointU32::new(band.min().x - band_width, 0),
        Direction::TopToBottom => PointU32::new(0, band.max().y),
        Direction::BottomToTop => PointU32::new(0, band.min().y - band_height),
    };

    BoundingBox::from_dimensions_and_origin(&band_new_origin, band_width, band_height)
}
