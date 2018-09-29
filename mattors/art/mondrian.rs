//! Generate some art inspired by `Composition in Red, Blue and Yellow` by
//! [Mondrian](https://en.wikipedia.org/wiki/Piet_Mondrian).

use rand::Rng;

use geo::{utils::clamp, BoundingBox, PointU32};

use crate::art::random_bbox_subdivisions;
use crate::drawing::{Drawer, NoopBlender};

/// Generate some Mondrian inspired artwork.
pub fn generate(
    img: &mut image::RgbImage,
    iterations: usize,
    minimum_area: u32,
    white: image::Rgb<u8>,
    fill_palette: &[image::Rgb<u8>],
    border_thickness: u32,
) {
    let mut rng = rand::thread_rng();

    let mut drawer = Drawer::new_with_no_blending(img);

    let (width, height) = drawer.dimensions();

    let rects = random_bbox_subdivisions(
        iterations,
        BoundingBox::from_dimensions(width, height),
        minimum_area,
        &mut rng,
    ).collect::<Vec<_>>();

    let mut draw_rect = |rect, pix| {
        drawer.rect(rect, &pix);
        draw_borders(&mut drawer, rect, border_thickness);
    };

    for rect in &rects {
        draw_rect(rect, white);
    }

    if !rects.is_empty() {
        let k = rng.gen_range(0, fill_palette.len() + 1);

        for pix in &fill_palette[..k] {
            let r = rng.gen_range(0, rects.len());

            draw_rect(&rects[r], *pix);
        }
    }
}

// TODO: drawing borders should be done by the drawing mod.
fn draw_borders(
    drawer: &mut Drawer<image::RgbImage, NoopBlender>,
    rect: &BoundingBox<u32>,
    border_thickness: u32,
) {
    let (width, height) = drawer.dimensions();

    let horizontal_band_width = rect.width().unwrap();
    let vertical_band_height = clamp(
        i64::from(rect.height().unwrap()) - i64::from(border_thickness) * 2,
        0,
        height,
    );

    let borders = [
        BoundingBox::from_dimensions_and_origin(
            rect.min(),
            horizontal_band_width,
            border_thickness,
        ),
        BoundingBox::from_dimensions_and_origin(
            &PointU32::new(rect.min().x, rect.min().y + border_thickness),
            border_thickness,
            vertical_band_height,
        ),
        BoundingBox::from_dimensions_and_origin(
            &PointU32::new(
                clamp(
                    i64::from(rect.max().x) - i64::from(border_thickness),
                    0,
                    width,
                ),
                rect.min().y + border_thickness,
            ),
            border_thickness,
            vertical_band_height,
        ),
        BoundingBox::from_dimensions_and_origin(
            &PointU32::new(
                rect.min().x,
                clamp(
                    i64::from(rect.max().y) - i64::from(border_thickness),
                    0,
                    height,
                ),
            ),
            horizontal_band_width,
            border_thickness,
        ),
    ];

    for border in &borders {
        drawer.rect(border, &image::Rgb { data: [0, 0, 0] });
    }
}
