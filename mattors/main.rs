//! Create some generative art.
#![deny(missing_docs, warnings)]

use std::f64;
use std::num::ParseFloatError;
use std::path::PathBuf;
use std::str::FromStr;

use image::{GenericImage, GenericImageView};

use num::complex::{Complex64, ParseComplexError};

use structopt::StructOpt;

use geo::{PointF64, PointU32};

use matto::art::delaunay;
use matto::art::dithering;
use matto::art::dragon;
use matto::art::fractree;
use matto::art::julia::{FractalPoint, JuliaGenIter};
use matto::art::mondrian;
use matto::art::patchwork;
use matto::art::primi;
use matto::art::primi::Shape;
use matto::art::quantize;
use matto::art::runes;
use matto::art::sierpinski;
use matto::art::stippling;
use matto::art::voronoi;

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

    /// Generate some Sierpinski triangles.
    #[structopt(name = "sierpinski")]
    Sierpinski(Sierpinski),

    /// Reconstruct an image from simple geometric shapes.
    #[structopt(name = "primirs")]
    Primirs(Primirs),

    /// Generate a Fractal Tree.
    #[structopt(name = "fractal-tree")]
    FractalTree(FractalTree),

    /// Generate an alphabet of random rune like characters.
    #[structopt(name = "runes")]
    Runes(Runes),

    /// Generate something similar to a proper delaunay triangulation.
    #[structopt(name = "delaunay")]
    Delaunay(Delaunay),

    /// Generate some Voronoi Diagrams.
    #[structopt(name = "voronoi")]
    Voronoi(Voronoi),

    /// Generate some art according to the Patchwork algorithm.
    #[structopt(name = "patchwork")]
    Patchwork(Patchwork),

    /// Generate some stippling art.
    #[structopt(name = "stippling")]
    Stippling(Stippling),

    /// Generate some mondrian inspired art.
    #[structopt(name = "mondrian")]
    Mondrian(Mondrian),

    /// Generate some mondrian inspired art.
    #[structopt(name = "dither")]
    Dither(Dither),

    /// Generate some spider web likes shapes.
    #[structopt(name = "tangled-web")]
    TangledWeb(TangledWeb),
}

/// Julia Set settings.
#[derive(StructOpt, Debug)]
pub struct Julia {
    /// Number of iterations to run the check for for every pixel. The
    /// higher the better, but lower numbers make cool fractals
    /// nonetheless.
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
/// quantization. The algorithm implemented here is Median Cut.
#[derive(StructOpt, Debug)]
pub struct Quantize {
    /// Number of dividing steps the Median Cut algorithm should take. The
    /// number of output colors is 2 ^ divide_steps.
    #[structopt(short = "d", long = "divide-steps", default_value = "4")]
    divide_steps: u32,

    /// Where to write the quantized image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "quantized.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,

    /// Image to quantize.
    #[structopt(name = "FILE", parse(from_os_str))]
    img_path: PathBuf,
}

/// Draw a Sierpinski Triangle.
#[derive(StructOpt, Debug)]
pub struct Sierpinski {
    /// How many times to divide the triangle.
    #[structopt(short = "d", long = "divide-steps", default_value = "6")]
    divide_steps: usize,

    /// Draw a fancy Sierpinski Triangle.
    #[structopt(short = "f", long = "fancy")]
    fancy: bool,

    /// Where to write the output image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "sierpinski.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,

    /// Width of the output image.
    #[structopt(short = "w", long = "width", default_value = "1600")]
    width: u32,

    /// Height of the output image.
    #[structopt(short = "h", long = "height", default_value = "1600")]
    height: u32,
}

/// Port of primitive/primipy. Approximate an image by using simple geometric
/// shapes.
#[derive(StructOpt, Debug)]
pub struct Primirs {
    /// Number of shapes to generate into the image.
    #[structopt(short = "s", long = "shapes", default_value = "100")]
    nshapes: usize,

    /// Number of mutations to perform for a single shape before changing shape.
    #[structopt(short = "m", long = "mutations", default_value = "100")]
    nmutations: u32,

    /// delta in x that determines how big the shapes will be.
    #[structopt(long = "dx", default_value = "16")]
    dx: u32,

    /// delta in x that determines how big the shapes will be.
    #[structopt(long = "dy", default_value = "16")]
    dy: u32,

    /// Scale the original image down by this percentage so that's faster.
    #[structopt(long = "scale-down", default_value = "1")]
    scale_down: u32,

    /// Where to write the "primitized" image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "primitized.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,

    /// Image to "primitize".
    #[structopt(name = "FILE", parse(from_os_str))]
    img_path: PathBuf,
}

/// Generate some awesome Fractal Trees.
#[derive(StructOpt, Debug)]
pub struct FractalTree {
    /// Number of branch points in the fractal tree.
    #[structopt(short = "b", long = "branches", default_value = "10")]
    nbranches: u32,

    /// Angle to use to rotate branches in radians.
    #[structopt(short = "a", long = "a", default_value = "0.523599")]
    branching_angle_step: f64,

    /// Factor to be multiplied to the branch len to change the latter.
    #[structopt(short = "l", long = "branch-factor", default_value = "0.6")]
    branch_len_factor: f64,

    /// Width of the output image.
    #[structopt(short = "w", long = "width", default_value = "1600")]
    width: u32,

    /// Height of the output image.
    #[structopt(short = "h", long = "height", default_value = "1600")]
    height: u32,

    /// Where to write the fractal image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "fractree.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate an alphabet of random rune like characters.
#[derive(StructOpt, Debug)]
pub struct Runes {
    /// Number of characters in the alphabet.
    #[structopt(short = "c", long = "characters", default_value = "20")]
    ntiles: u32,

    /// Number of points the rune should have. The higher the more complex the
    /// rune will be.
    #[structopt(short = "p", long = "points", default_value = "3")]
    npoints: u32,

    /// Width of each rune.
    #[structopt(short = "w", long = "width", default_value = "128")]
    width: u32,

    /// Height of each rune.
    #[structopt(short = "h", long = "height", default_value = "128")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "runes.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate something similar to a proper delaunay triangulation.
#[derive(StructOpt, Debug)]
pub struct Delaunay {
    /// Size of the grid where to put points.
    #[structopt(short = "g", long = "grid-size", default_value = "25")]
    grid_size: u32,

    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "delaunay.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate some Voronoi Diagrams.
#[derive(StructOpt, Debug)]
pub struct Voronoi {
    /// Number of points used to generate the diagram.
    #[structopt(short = "p", long = "points", default_value = "50")]
    npoints: usize,

    /// Whether to use a gradient as the background of the image or randomly
    /// generated colors.
    #[structopt(short = "g", long = "gradient-background")]
    gradient_background: bool,

    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "voronoi.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate some art according to the PatchWork algorithm.
#[derive(StructOpt, Debug)]
pub struct Patchwork {
    /// Number of points used to calculate clusters and convex hulls.
    #[structopt(short = "p", long = "points", default_value = "5000")]
    npoints: usize,

    /// How many clusters to calculate at each step. Works best with low
    /// numbers.
    #[structopt(short = "c", long = "clusters", default_value = "3")]
    clusters: usize,

    /// Whether to fill the polygons of the last generation or not.
    #[structopt(short = "f", long = "fill-polygons")]
    fill_polygons: bool,

    /// How many iterations the algorithm should perform.
    #[structopt(short = "i", long = "iterations", default_value = "3")]
    iterations: usize,

    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "patchwork.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate some stippling art.
#[derive(StructOpt, Debug)]
pub struct Stippling {
    #[structopt(subcommand)]
    command: StipplingCommand,

    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "stippling.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Generate some stippling art.
#[derive(StructOpt, Debug)]
pub enum StipplingCommand {
    /// Stippling some bands to give a gradient-like look.
    #[structopt(name = "gradient")]
    Gradient(StipplingGradient),

    /// Stippling some rectangles.
    #[structopt(name = "rects")]
    StipplingRects(StipplingRects),
}

/// Stippling some bands to give a gradient-like look.
#[derive(StructOpt, Debug)]
pub struct StipplingGradient {
    /// Number of bands in the gradient.
    #[structopt(short = "b", long = "bands", default_value = "5")]
    bands: u32,

    /// The number of points for the first band.
    #[structopt(short = "p", long = "point", default_value = "5000")]
    first_band_points: u32,

    /// The grow coefficient is the factor that determines the number of points
    /// in the next band. In particular npoints = prev_points + prev_points * k.
    #[structopt(short = "k", long = "grow-coefficient", default_value = "2")]
    grow_coeff: u32,
}

/// Stippling some rectangles.
#[derive(StructOpt, Debug)]
pub struct StipplingRects {
    /// Number of iterations to divive the given image of.
    #[structopt(short = "i", long = "iterations", default_value = "500")]
    iterations: usize,

    /// The number of points in each rectangle.
    #[structopt(short = "p", long = "point", default_value = "300")]
    points: u32,

    /// The minimum area each rectangle must have in order to recurse in it.
    #[structopt(short = "a", long = "minimum-area", default_value = "1000")]
    minimum_area: u32,
}

/// Generate some art inspired by Mondrian's Composition in Red, Blue and
/// Yellow.
#[derive(StructOpt, Debug)]
pub struct Mondrian {
    /// Number of iterations to divive the given image of.
    #[structopt(short = "i", long = "iterations", default_value = "5")]
    iterations: usize,

    /// The minimum area each rectangle must have in order to recurse in it.
    #[structopt(short = "a", long = "minimum-area", default_value = "1000")]
    minimum_area: u32,

    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Where to write the final image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "mondrian.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
}

/// Dither a given image.
#[derive(StructOpt, Debug)]
pub struct Dither {
    /// Number of colors in the resulting image.
    #[structopt(short = "c", long = "colors", default_value = "5")]
    levels: u8,

    /// Convert the image to rgb before dithering.
    #[structopt(long = "rgb")]
    rgb: bool,

    /// Where to write the dithered image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "dithered.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,

    /// Image to dither.
    #[structopt(name = "FILE", parse(from_os_str))]
    img_path: PathBuf,
}

/// Generate 2d tangled webs inspired by https://inconvergent.net/2019/a-tangle-of-webs/
#[derive(StructOpt, Debug)]
pub struct TangledWeb {
    /// Width of the image.
    #[structopt(short = "w", long = "width", default_value = "1920")]
    width: u32,

    /// Height of the image.
    #[structopt(short = "h", long = "height", default_value = "1080")]
    height: u32,

    /// Number of iterations to divive the given image of.
    #[structopt(short = "i", long = "iterations", default_value = "1000")]
    iterations: usize,

    /// Number of divisions to break the initial circle into.
    #[structopt(short = "d", long = "circle-divisions", default_value = "30")]
    circle_divisions: u8,

    /// Whether to save the image as an svg or png.
    #[structopt(long = "svg")]
    svg: bool,

    /// Where to write the dithered image.
    #[structopt(
        short = "o",
        long = "output",
        default_value = "tangled-web.png",
        parse(from_os_str)
    )]
    output_path: PathBuf,
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
        Command::Sierpinski(ref config) => spawn_sierpinski(config),
        Command::Primirs(ref config) => primirs(config),
        Command::FractalTree(ref config) => fractal_tree(config),
        Command::Runes(ref config) => runes(config),
        Command::Delaunay(ref config) => delaunay(config),
        Command::Voronoi(ref config) => voronoi(config),
        Command::Patchwork(ref config) => patchwork(config),
        Command::Stippling(ref config) => stippling(config),
        Command::Mondrian(ref config) => mondrian(config),
        Command::Dither(ref config) => dither(config),
        Command::TangledWeb(ref config) => tangled_web(config),
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
    F: Fn(Complex64, u32) -> FractalPoint,
{
    let stepx = (end.x - start.x) / f64::from(config.width);
    let stepy = (end.y - start.y) / f64::from(config.height);

    let frac_it = JuliaGenIter::new(
        *start,
        config.width,
        config.height,
        stepx,
        stepy,
        config.iterations,
        gen,
    );

    println!("Fractal: {}", name);

    let imgbuf = frac_it
        .into_image()
        .expect("error while generating fractal");
    let img = image::ImageRgb8(imgbuf);

    // let img = img.resize_exact(width, height, image::Lanczos3);

    img.save(&format!("{}.png", name))
        .expect("cannot save output image");
}

fn spawn_dragons(iterations: u32) {
    println!("Dragons!");

    let red = dragon::dragon(iterations, dragon::Move::Left);
    let red_img = dragon::dragon_to_image(&red, 1920, 1080, 1480, 730, 2, [255, 0, 0]);

    let blue = dragon::dragon(iterations, dragon::Move::Up);
    let blue_img = dragon::dragon_to_image(&blue, 1920, 1080, 500, 730, 2, [0, 0, 255]);

    let green = dragon::dragon(iterations, dragon::Move::Right);
    let green_img = dragon::dragon_to_image(&green, 1920, 1080, 500, 350, 2, [0, 255, 0]);

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
    let red_img = dragon::dragon_to_image(&red, 1920, 1080, 1480, 530, 2, RED);

    let blue = dragon::horns(iterations, dragon::Move::Up);
    let blue_img = dragon::dragon_to_image(&blue, 1920, 1080, 550, 790, 2, DARK_BLUE);

    let green = dragon::horns(iterations, dragon::Move::Right);
    let green_img = dragon::dragon_to_image(&green, 1920, 1080, 960, 550, 2, LIGHT_GREEN);

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
    let rgb = img
        .as_rgb8()
        .expect("cannot convert source image to rgb8 image");

    let res = quantize::quantize(rgb.pixels().cloned(), config.divide_steps);

    let mut quantized = rgb.clone();
    for pixel in quantized.pixels_mut() {
        *pixel = res.quantized_pixels[pixel];
    }

    quantized
        .save(&config.output_path)
        .expect("cannot save quantized file");
}

fn spawn_sierpinski(config: &Sierpinski) {
    let mut img = image::RgbImage::from_pixel(
        config.width,
        config.height,
        image::Rgb {
            data: [0x40, 0xbe, 0xcd],
        },
    );

    if config.fancy {
        sierpinski::fancy_sierpinski(
            &mut img,
            config.divide_steps,
            false,
            &[
                image::Rgb {
                    data: [0x02, 0x44, 0x0c],
                },
                image::Rgb {
                    data: [0x78, 0x94, 0x00],
                },
                image::Rgb {
                    data: [0xe4, 0xd5, 0x65],
                },
                image::Rgb {
                    data: [0xf3, 0xf5, 0xe7],
                },
            ],
        );
    } else {
        sierpinski::fancy_sierpinski(
            &mut img,
            config.divide_steps,
            true,
            &[image::Rgb {
                data: [0xf3, 0xf5, 0xe7],
            }],
        );
    }

    img.save(&config.output_path)
        .expect("cannot save sierpinski triangle");
}

fn primirs(config: &Primirs) {
    let img = image::open(&config.img_path).expect("cannot open source image file");
    let rgba = img.to_rgba();

    let primitized = if config.scale_down > 1 {
        let resized = image::imageops::resize(
            &rgba,
            img.width() / config.scale_down,
            img.height() / config.scale_down,
            image::Triangle,
        );

        primi::primify::<_, geo::Triangle<u32>>(
            &resized,
            config.nshapes,
            config.nmutations,
            config.dx,
            config.dy,
        )
        .map(|prim| {
            let mut upscaled_img =
                image::RgbaImage::from_pixel(rgba.width(), rgba.height(), prim.dominant_color);

            for shape in prim.shapes {
                let upscaled_shape = shape.upscale(config.scale_down);

                upscaled_shape.draw(&rgba, &mut upscaled_img);
            }

            (upscaled_img, prim.best_error)
        })
    } else {
        primi::primify::<_, geo::Triangle<u32>>(
            &rgba,
            config.nshapes,
            config.nmutations,
            config.dx,
            config.dy,
        )
        .map(|prim| (prim.best_image, prim.best_error))
    };

    let (best_image, best_error) = primitized.expect("primirs error");

    println!("best error {:?}", best_error);

    best_image
        .save(&config.output_path)
        .expect("cannot save primitized file");
}

fn fractal_tree(config: &FractalTree) {
    let mut img =
        image::GrayImage::from_pixel(config.width, config.height, image::Luma { data: [0] });

    fractree::fractal_tree(
        &mut img,
        config.nbranches,
        PointU32::new(config.width / 2, config.height - 1),
        -f64::consts::PI / 2.0,
        config.branching_angle_step,
        f64::from(config.height) / 3.0,
        config.branch_len_factor,
        &image::Luma { data: [0xFF] },
    );

    img.save(&config.output_path).expect("cannot save image");
}

fn runes(config: &Runes) {
    let white_pix = image::Luma { data: [0xFF] };
    let black_pix = image::Luma { data: [0] };

    let mut imgbuf =
        image::GrayImage::from_pixel(config.ntiles * config.width, config.height, white_pix);

    for i in 0..config.ntiles {
        let mut rune = imgbuf.sub_image(i * config.width, 0, config.width, config.height);
        runes::draw_random_rune(&mut rune, config.npoints, &black_pix);
    }

    imgbuf.save(&config.output_path).expect("cannot save image");
}

fn delaunay(config: &Delaunay) {
    let mut color_config = matto::color::RandomColorConfig::new()
        .hue(matto::color::KnownHue::Blue)
        .luminosity(matto::color::Luminosity::Light);

    let alpha = 0xd6;

    let mut img = image::RgbaImage::from_pixel(
        config.width,
        config.height,
        image::Rgba {
            data: matto::color::random_color(&mut color_config).to_rgba(alpha),
        },
    );

    delaunay::random_triangulation(&mut img, &mut color_config, config.grid_size, alpha);

    img.save(&config.output_path).expect("cannot save image");
}

fn voronoi(config: &Voronoi) {
    let mut color_config =
        matto::color::RandomColorConfig::new().luminosity(matto::color::Luminosity::Bright);

    let mut img = image::RgbImage::new(config.width, config.height);

    if config.gradient_background {
        let color1 = matto::color::random_color(&mut color_config).to_rgb();
        let color2 = matto::color::random_color(&mut color_config).to_rgb();

        voronoi::gradient_voronoi(
            &mut img,
            image::Rgb { data: color1 },
            image::Rgb { data: color2 },
            config.npoints,
        )
    } else {
        voronoi::random_voronoi(&mut img, &mut color_config, config.npoints);
    }

    img.save(&config.output_path).expect("cannot save image");
}

fn patchwork(config: &Patchwork) {
    let mut img = image::RgbImage::new(config.width, config.height);

    patchwork::random_patchwork(
        &mut img,
        config.npoints,
        config.clusters,
        config.iterations,
        config.fill_polygons,
    );

    img.save(&config.output_path).expect("cannot save image");
}

fn stippling(config: &Stippling) {
    let mut img = image::RgbImage::from_pixel(
        config.width,
        config.height,
        image::Rgb {
            data: [0xFF, 0xFF, 0xFF],
        },
    );

    match config.command {
        StipplingCommand::Gradient(ref gradient_config) => {
            stippling::gradient(
                &mut img,
                gradient_config.bands,
                gradient_config.first_band_points,
                gradient_config.grow_coeff,
                image::Rgb { data: [0, 0, 0] },
                stippling::Direction::TopToBottom,
            );
        }
        StipplingCommand::StipplingRects(ref rects_config) => {
            stippling::rects(
                &mut img,
                rects_config.iterations,
                rects_config.points,
                rects_config.minimum_area,
                image::Rgb { data: [0, 0, 0] },
            );
        }
    }

    img.save(&config.output_path).expect("cannot save image");
}

fn mondrian(config: &Mondrian) {
    let mut img = image::RgbImage::new(config.width, config.height);

    let fill_palette = [
        image::Rgb {
            data: [0x8d, 0x22, 0x02],
        },
        image::Rgb {
            data: [0x0b, 0x18, 0x3b],
        },
        image::Rgb {
            data: [0xd0, 0x95, 0x02],
        },
    ];

    mondrian::generate(
        &mut img,
        config.iterations,
        config.minimum_area,
        image::Rgb {
            data: [0xe6, 0xeb, 0xc3],
        },
        &fill_palette,
        10,
    );

    img.save(&config.output_path).expect("cannot save image");
}

fn dither(config: &Dither) {
    let img = image::open(&config.img_path).expect("cannot load image file");

    let step = u8::max_value() / config.levels;

    if config.rgb {
        let dithered = dithering::dither(&img.to_rgb(), |l| image::Rgb {
            data: {
                [
                    l.data[0] / step * step,
                    l.data[1] / step * step,
                    l.data[2] / step * step,
                ]
            },
        });

        dithered
            .save(&config.output_path)
            .expect("cannot save image");
    } else {
        let dithered = dithering::dither(&img.to_luma(), |l| image::Luma {
            data: { [l.data[0] / step * step] },
        });

        dithered
            .save(&config.output_path)
            .expect("cannot save image");
    }
}

fn tangled_web(config: &TangledWeb) {
    if !config.svg {
        let mut img = image::RgbImage::new(config.width, config.height);

        matto::art::tangled_web::generate_img(&mut img, config.iterations, config.circle_divisions);

        img.save(&config.output_path).expect("cannot save image");
        return;
    }

    let mut f = std::fs::File::create(config.output_path.with_extension("svg")).unwrap();
    matto::art::tangled_web::generate_svg(
        &mut f,
        (config.width, config.height),
        config.iterations,
        config.circle_divisions,
    )
    .expect("error writing svg");
}
