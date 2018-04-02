//! Create some generative art.
#![deny(missing_docs, warnings)]

#[macro_use]
extern crate structopt;

extern crate image;
extern crate matto;
extern crate num;

use std::fs::File;
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::str::FromStr;

use num::complex::{Complex64, ParseComplexError};

use structopt::StructOpt;

use matto::dragon;
use matto::julia::{fractal_to_image, gen_fractal, FractalPoint};
use matto::point::PointF64;
use matto::quantize;

const LIGHT_GREEN: [u8; 3] = [0x17, 0xB9, 0x78];
const RED: [u8; 3] = [0xF6, 0x72, 0x80];
const DARK_BLUE: [u8; 3] = [0x1D, 0x27, 0x86];

fn parse_complex(s: &str) -> Result<Complex64, ParseComplexError<ParseFloatError>> {
    Complex64::from_str(s.trim())
}

/// Have fun with some generative art
#[derive(StructOpt, Debug)]
#[structopt(name = "matto")]
pub enum Command {
    /// Generate the dragon fractals.
    #[structopt(name = "dragons")]
    Dragons {
        /// How many iterations the algorithm should perform before creating the image.
        #[structopt(short = "i", long = "iterations", default_value = "17")]
        iterations: u32,
    },

    /// Generate the horns fractals which are invented by me(really?) which are
    /// a slight modification of `Dragons`. It's also full of little smiles :).
    #[structopt(name = "horns")]
    Horns {
        /// How many iterations the algorithm should perform before creating the image.
        #[structopt(short = "i", long = "iterations", default_value = "16")]
        iterations: u32,
    },

    /// Generate some julia fractals. The Mandelbrot set is one of those.
    #[structopt(name = "julia")]
    Julia(Julia),

    /// Quantize an image.
    #[structopt(name = "quantize")]
    Quantize(Quantize),
}

/// Julia Set settings.
#[derive(StructOpt, Debug)]
pub struct Julia {
    /// Number of iterations to run the check for for every pixel. The
    /// higher the better, but lower numbers make cool fractals
    /// nonentheless.
    #[structopt(short = "i", long = "iterations", default_value = "64")]
    iterations: u32,

    /// Width of the output image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the output image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Which Julia set to generate.
    #[structopt(subcommand)]
    set_type: Option<JuliaSet>,
}

/// All the available Julia sets.
#[derive(StructOpt, Debug)]
pub enum JuliaSet {
    /// Generate all the Julia fractals.
    #[structopt(name = "all")]
    All,

    /// Generate the Mandelbrot set
    #[structopt(name = "mandelbrot")]
    Mandelbrot,

    /// Generate a Planets like fractal.
    #[structopt(name = "planets")]
    Planets,

    /// Generate a dragon like fractal.
    #[structopt(name = "dragon-like")]
    DragonLikeSpiral,

    /// Generate a black holes like fractal.
    #[structopt(name = "black-holes")]
    BlackHoles,

    /// Generate custom fractal by specifying its parameters.
    #[structopt(name = "custom")]
    Custom {
        /// Top left point where to start the generation.
        #[structopt(short = "s", long = "start")]
        start: PointF64,

        /// Bottom right point where to end the generation.
        #[structopt(short = "e", long = "end")]
        end: PointF64,

        /// The C constant in a Julia set.
        #[structopt(short = "c", parse(try_from_str = "parse_complex"))]
        c: Complex64,

        /// Name of the fractal.
        #[structopt(short = "n", long = "name", default_value = "custom")]
        name: String,
    },
}

/// Reduce the number of colors an image uses. This process is called
/// quantization. The algorithm implemented here is [Median
/// Cut](https://en.wikipedia.org/wiki/Median_cut).
#[derive(StructOpt, Debug)]
pub struct Quantize {
    /// Number of dividing steps the Median Cut algorithm should take. The
    /// number of output colors is 2 ^ divide_steps.
    #[structopt(short = "d", long = "divide-steps", default_value = "4")]
    divide_steps: u32,

    /// Where to write the quantized image.
    #[structopt(short = "o", long = "output", default_value = "quantized.png", parse(from_os_str))]
    output_path: PathBuf,

    /// Image to quantize.
    #[structopt(name = "FILE", parse(from_os_str))]
    img_path: PathBuf,
}

fn main() {
    let command = Command::from_args();

    match command {
        Command::Dragons { iterations } => spawn_dragons(iterations),
        Command::Horns { iterations } => spawn_horns(iterations),
        Command::Julia(ref config) => match config.set_type {
            None | Some(JuliaSet::All) => {
                mandelbrot(config);
                planets(config);
                dragon_like(config);
                black_holes(config);
            }
            Some(JuliaSet::Mandelbrot) => mandelbrot(config),
            Some(JuliaSet::Planets) => planets(config),
            Some(JuliaSet::DragonLikeSpiral) => dragon_like(config),
            Some(JuliaSet::BlackHoles) => black_holes(config),
            Some(JuliaSet::Custom {
                ref start,
                ref end,
                ref c,
                ref name,
            }) => create_julia_set(config, name, start, end, |f, it| {
                FractalPoint::julia(f, *c, it)
            }),
        },
        Command::Quantize(ref config) => quantize_image(config),
    }
}

fn mandelbrot(config: &Julia) {
    create_julia_set(
        config,
        "mandelbrot",
        &PointF64::new(-3.0, -1.2),
        &PointF64::new(1.0, 1.2),
        FractalPoint::mandelbrot,
    );
}

fn planets(config: &Julia) {
    let c = Complex64::new(-0.4, 0.6);

    create_julia_set(
        config,
        "planets",
        &PointF64::new(-3.0, -1.2),
        &PointF64::new(2.0, 1.2),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn dragon_like(config: &Julia) {
    let c = Complex64::new(-0.8, 0.156);

    create_julia_set(
        config,
        "dragon_like",
        &PointF64::new(-3.0, -1.2),
        &PointF64::new(2.0, 1.2),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn black_holes(config: &Julia) {
    let c = Complex64::new(0.285, 0.01);

    create_julia_set(
        config,
        "black_holes",
        &PointF64::new(-1.2, -1.2),
        &PointF64::new(1.2, 1.0),
        |f, it| FractalPoint::julia(f, c, it),
    );
}

fn create_julia_set<F>(config: &Julia, name: &str, start: &PointF64, end: &PointF64, gen: F)
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

fn spawn_dragons(iterations: u32) {
    println!("Dragons!");

    let red = dragon::dragon(iterations, dragon::Move::Left);
    let red_img = dragon::dragon_to_image(&red, 1920, 1080, 1480, 730, 2, &[255, 0, 0]);

    let blue = dragon::dragon(iterations, dragon::Move::Up);
    let blue_img = dragon::dragon_to_image(&blue, 1920, 1080, 500, 730, 2, &[0, 0, 255]);

    let green = dragon::dragon(iterations, dragon::Move::Right);
    let green_img = dragon::dragon_to_image(&green, 1920, 1080, 500, 350, 2, &[0, 255, 0]);

    let redblue_img = overlap_images(&red_img, &blue_img).unwrap();
    let rgb_img = overlap_images(&redblue_img, &green_img).unwrap();

    red_img.save("red-dragon.png").unwrap();
    blue_img.save("blue-dragon.png").unwrap();
    green_img.save("green-dragon.png").unwrap();
    redblue_img.save("redblue-dragon.png").unwrap();
    rgb_img.save("rgb-dragon.png").unwrap();
}

fn spawn_horns(iterations: u32) {
    println!("Horns!");

    let red = dragon::horns(iterations, dragon::Move::Left);
    let red_img = dragon::dragon_to_image(&red, 1920, 1080, 1480, 530, 2, &RED);

    let blue = dragon::horns(iterations, dragon::Move::Up);
    let blue_img = dragon::dragon_to_image(&blue, 1920, 1080, 550, 790, 2, &DARK_BLUE);

    let green = dragon::horns(iterations, dragon::Move::Right);
    let green_img = dragon::dragon_to_image(&green, 1920, 1080, 960, 550, 2, &LIGHT_GREEN);

    red_img.save("red-horns.png").unwrap();
    blue_img.save("blue-horns.png").unwrap();
    green_img.save("green-horns.png").unwrap();
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
                (lhs_pix[0].saturating_add(rhs_pix[0])) / 2,
                (lhs_pix[1].saturating_add(rhs_pix[1])) / 2,
                (lhs_pix[2].saturating_add(rhs_pix[2])) / 2,
            ];

            res.put_pixel(x, y, image::Rgb(new_pix));
        }
    }

    Some(res)
}

fn quantize_image(config: &Quantize) {
    let img = image::open(&config.img_path).expect("cannot open source image file");
    let rgb = img.as_rgb8()
        .expect("cannot convert source image to rgb8 image");

    let res =
        quantize::quantize(rgb.pixels().cloned(), config.divide_steps).expect("quantization error");

    let mut quantized = rgb.clone();
    for pixel in quantized.pixels_mut() {
        *pixel = res.quantized_pixels[pixel];
    }

    quantized
        .save(&config.output_path)
        .expect("cannot save quantized file");
}
