//! Simple module to generate some rune like characters
extern crate image;
extern crate rand;

use std::fmt::Debug;

use self::rand::Rng;

use drawing::Drawer;
use geo::PointU32;

#[derive(Debug)]
enum Simmetry {
    Horizontal,
    None,
    Vertical,
    VerticalAndHorizontal,
}

impl Simmetry {
    fn random<'a, R: Rng>(rng: &mut R) -> &'a Self {
        rng.choose(&[
            Simmetry::Horizontal,
            Simmetry::None,
            Simmetry::Vertical,
            Simmetry::VerticalAndHorizontal,
        ]).unwrap()
    }

    fn divide(&self, width: u32, height: u32) -> (u32, u32) {
        match *self {
            Simmetry::Horizontal => (width, height / 2),
            Simmetry::None => (width, height),
            Simmetry::Vertical => (width / 2, height),
            Simmetry::VerticalAndHorizontal => (width / 2, height / 2),
        }
    }

    fn mirror_image<I: image::GenericImage>(
        &self,
        img: &mut I,
        simmetry_dimensions: (u32, u32),
        img_dimensions: (u32, u32),
    ) {
        let (simmetry_width, simmetry_height) = simmetry_dimensions;
        let (width, height) = img_dimensions;

        match *self {
            Simmetry::None => {}
            Simmetry::Horizontal => {
                for y in 0..simmetry_height {
                    for x in 0..simmetry_width {
                        let p = img.get_pixel(x, y);
                        img.put_pixel(x, height - y - 1, p);
                    }
                }
            }
            Simmetry::Vertical => {
                for y in 0..simmetry_height {
                    for x in 0..simmetry_width {
                        let p = img.get_pixel(x, y);
                        img.put_pixel(width - x - 1, y, p);
                    }
                }
            }
            Simmetry::VerticalAndHorizontal => {
                for y in 0..simmetry_height {
                    for x in 0..simmetry_width {
                        let p = img.get_pixel(x, y);

                        img.put_pixel(width - x - 1, y, p);
                        img.put_pixel(x, height - y - 1, p);
                        img.put_pixel(width - x - 1, height - y - 1, p);
                    }
                }
            }
        }
    }
}

/// Draw a rune like shape onto the given image.
pub fn draw_random_rune<I>(rune: &mut I, npoints: u32, fg_color: &I::Pixel)
where
    I: image::GenericImage,
    I::Pixel: Debug,
    f64: From<<I::Pixel as image::Pixel>::Subpixel>,
{
    let mut rng = rand::thread_rng();

    let simmetry = Simmetry::random(&mut rng);

    let (rune_width, rune_height) = rune.dimensions();
    let (rune_quad_width, rune_quad_height) = simmetry.divide(rune_width, rune_height);

    {
        let mut quad = rune.sub_image(0, 0, rune_quad_width, rune_quad_height);
        draw_random_rune_quad(&mut quad, npoints, fg_color, &mut rng);
    }

    simmetry.mirror_image(
        rune,
        (rune_quad_width, rune_quad_height),
        (rune_width, rune_height),
    );

    if rng.gen_bool(0.75) {
        let mut drawer = Drawer::new_with_no_blending(rune);

        let mw = rune_width / 2;
        drawer.line(
            PointU32::new(mw, 0),
            PointU32::new(mw, rune_height),
            fg_color,
        );
    }
}

fn draw_random_rune_quad<I, R>(quad: &mut I, npoints: u32, fg_color: &I::Pixel, rng: &mut R)
where
    I: image::GenericImage,
    I::Pixel: Debug,
    R: Rng,
    f64: From<<I::Pixel as image::Pixel>::Subpixel>,
{
    let mut drawer = Drawer::new_with_no_blending(quad);

    let (quad_width, quad_height) = drawer.dimensions();
    let mw = quad_width / 2;

    (0..npoints).fold(
        PointU32::new(mw, rng.gen_range(0, quad_height)),
        |last_point, _| {
            let x = rng.gen_range(mw, quad_width);
            let y = rng.gen_range(0, quad_height);

            let p = PointU32::new(x, y);
            drawer.antialiased_line(last_point, p, fg_color);

            p
        },
    );
}
