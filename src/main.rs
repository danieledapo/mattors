#[macro_use]
extern crate structopt;

extern crate image;
extern crate matto;
extern crate num;

use std::fs::File;
use std::num::ParseFloatError;
use std::str::FromStr;

use structopt::StructOpt;

use num::complex::{Complex64, ParseComplexError};

use matto::julia::{fractal_to_image, gen_fractal, FractalPoint, Point};
use matto::dragon;

fn parse_complex(s: &str) -> Result<Complex64, ParseComplexError<ParseFloatError>> {
    Complex64::from_str(s.trim())
}

/// Have fun with some generative art
#[derive(StructOpt, Debug)]
#[structopt(name = "matto")]
pub enum Command {
    #[structopt(name = "dragons")]
    /// Generate the dragon fractals
    Dragons,

    #[structopt(name = "julia")]
    /// Generate some julia fractals. The Mandelbrot set is one of those.
    Julia(Julia),
}

#[derive(StructOpt, Debug)]
pub struct Julia {
    #[structopt(short = "i", long = "iterations", default_value = "64")]
    /// Number of iterations to run the check for for every pixel. The
    /// higher the better, but lower numbers make cool fractals
    /// nonentheless.
    iterations: u32,

    #[structopt(short = "w", long = "width", default_value = "1920")]
    /// Width of the output image.
    width: u32,

    #[structopt(short = "h", long = "height", default_value = "1080")]
    /// Height of the output image.
    height: u32,

    #[structopt(subcommand)]
    /// Which Julia set to generate.
    set_type: Option<JuliaSet>,
}

/// Have fun with some generative art
#[derive(StructOpt, Debug)]
pub enum JuliaSet {
    #[structopt(name = "all")]
    All,

    #[structopt(name = "mandelbrot")]
    Mandelbrot,

    #[structopt(name = "planets")]
    Planets,

    #[structopt(name = "dragon-like")]
    DragonLikeSpiral,

    #[structopt(name = "black-holes")]
    BlackHoles,

    #[structopt(name = "custom")]
    /// Generate custom fractal by specifying its parameters.
    Custom {
        #[structopt(short = "s", long = "start")]
        /// Top left point where to start the generation.
        start: Point,

        #[structopt(short = "e", long = "end")]
        // / Bottom right point where to end the generation.
        end: Point,

        #[structopt(short = "c", parse(try_from_str = "parse_complex"))]
        /// The C constant in a Julia set.
        c: Complex64,
    },
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::Dragons => spawn_dragons(),
        Command::Julia(ref config) => match &config.set_type {
            &None | &Some(JuliaSet::All) => {
                mandelbrot(&config);
                planets(&config);
                dragon_like(&config);
                black_holes(&config);
            }
            &Some(JuliaSet::Mandelbrot) => mandelbrot(&config),
            &Some(JuliaSet::Planets) => planets(&config),
            &Some(JuliaSet::DragonLikeSpiral) => planets(&config),
            &Some(JuliaSet::BlackHoles) => black_holes(&config),
            &Some(JuliaSet::Custom {
                ref start,
                ref end,
                ref c,
            }) => create_julia_set(&config, "custom", start, end, |f, it| {
                FractalPoint::julia(f, *c, it)
            }),
        },
    }
}

fn mandelbrot(config: &Julia) {
    create_julia_set(
        config,
        "mandelbrot",
        &Point::new(-3.0, -1.2),
        &Point::new(1.0, 1.2),
        FractalPoint::mandelbrot,
    );
}

fn planets(config: &Julia) {
    let c = Complex64::new(-0.4, 0.6);

    create_julia_set(
        config,
        "planets",
        &Point::new(-3.0, -1.2),
        &Point::new(2.0, 1.2),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn dragon_like(config: &Julia) {
    let c = Complex64::new(-0.8, 0.156);

    create_julia_set(
        config,
        "dragon_like",
        &Point::new(-3.0, -1.2),
        &Point::new(2.0, 1.2),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn black_holes(config: &Julia) {
    let c = Complex64::new(0.285, 0.01);

    create_julia_set(
        config,
        "black_holes",
        &Point::new(-1.2, -1.2),
        &Point::new(1.2, 1.0),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn create_julia_set<F>(config: &Julia, name: &str, start: &Point, end: &Point, gen: F)
where
    F: Sync + Send + Fn(Complex64, u32) -> FractalPoint,
{
    let stepx = (end.x - start.x) / f64::from(config.width);
    let stepy = (end.y - start.y) / f64::from(config.height);

    let frac = gen_fractal(
        start,
        config.width,
        config.height,
        stepx,
        stepy,
        config.iterations,
        gen,
    );

    println!("Fractal: {}", name);

    let img = fractal_to_image(&frac);
    // let img = img.resize_exact(width, height, image::Lanczos3);

    let mut fout = &File::create(&format!("{}.png", name)).unwrap();
    img.save(&mut fout, image::PNG).unwrap();
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
