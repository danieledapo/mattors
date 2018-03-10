#[macro_use]
extern crate structopt;

extern crate image;
extern crate matto;
extern crate num;

use structopt::StructOpt;

use num::complex::Complex64;

use std::fs::File;

use matto::julia::{fractal_to_image, gen_fractal, Bound, FractalPoint};
use matto::dragon;

/// Have fun with some generative art
#[derive(StructOpt, Debug)]
#[structopt(name = "matto")]
pub enum Command {

    #[structopt(name = "dragons")]
    /// Generate the dragon fractals
    Dragons,

    #[structopt(name = "julia")]
    /// Generate some julia fractals. The Mandelbrot set is one of those.
    Julia,
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::Dragons => spawn_dragons(),
        Command::Julia => julia_fractals(),
    }
}

fn julia_fractals() {
    let manifest = vec![
        (Bound::new(-3.0, -1.2), Bound::new(1.0, 1.2), None),
        (
            Bound::new(-3.0, -1.2),
            Bound::new(2.0, 1.2),
            Some(Complex64::new(-0.4, 0.6)),
        ),
        (
            Bound::new(-3.0, -1.2),
            Bound::new(2.0, 1.2),
            Some(Complex64::new(-0.8, 0.156)),
        ),
        (
            Bound::new(-1.2, -1.2),
            Bound::new(1.2, 1.0),
            Some(Complex64::new(0.285, 0.01)),
        ),
    ];

    for (i, row) in manifest.iter().enumerate() {
        let (ref start, ref end, ref c) = *row;

        let frac = {
            if let Some(c) = *c {
                gen_fractal(start, end, 0.002, |f| FractalPoint::julia(f, c))
            } else {
                gen_fractal(start, end, 0.002, FractalPoint::mandelbrot)
            }
        };

        println!("Fractal: {}", i + 1);

        let img = fractal_to_image(&frac);
        let img = img.resize_exact(1920, 1080, image::Lanczos3);

        let mut fout = &File::create(&format!("{}.png", i + 1)).unwrap();
        img.save(&mut fout, image::PNG).unwrap();
    }
}

fn spawn_dragons() {
    println!("Dragons!");

    let red = dragon::dragon(17, dragon::Move::Left);
    let red_img = dragon::dragon_to_image(&red, 1920, 1080, 1480, 730, 2, &[255, 0, 0]);

    let blue = dragon::dragon(17, dragon::Move::Up);
    let blue_img = dragon::dragon_to_image(&blue, 1920, 1080, 500, 730, 2, &[0, 0, 255]);

    let green = dragon::dragon(17, dragon::Move::Right);
    let green_img = dragon::dragon_to_image(&green, 1920, 1080, 500, 350, 2, &[0, 255, 0]);

    let redblue_img = overlap_images(&red_img, &blue_img).unwrap();
    let rgb_img = overlap_images(&redblue_img, &green_img).unwrap();

    red_img.save("red-dragon.png").unwrap();
    blue_img.save("blue-dragon.png").unwrap();
    green_img.save("green-dragon.png").unwrap();
    redblue_img.save("redblue-dragon.png").unwrap();
    rgb_img.save("rgb-dragon.png").unwrap();
}

fn overlap_images(lhs: &image::RgbImage, rhs: &image::RgbImage) -> Option<image::RgbImage> {
    if lhs.width() != rhs.width() || lhs.height() != rhs.height() {
        return None;
    }

    let mut res = image::ImageBuffer::new(lhs.width(), rhs.height());

    for x in 0..lhs.width() {
        for y in 0..lhs.height() {
            let lhs_pix = lhs.get_pixel(x, y).data;
            let rhs_pix = rhs.get_pixel(x, y).data;

            let new_pix = [
                (lhs_pix[0] + rhs_pix[0]) / 2,
                (lhs_pix[1] + rhs_pix[1]) / 2,
                (lhs_pix[2] + rhs_pix[2]) / 2,
            ];

            res.put_pixel(x, y, image::Rgb(new_pix));
        }
    }

    Some(res)
}
