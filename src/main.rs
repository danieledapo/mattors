extern crate image;
extern crate num;
extern crate rayon;

use std::fs::File;

use image::GenericImage;
use num::complex::Complex64;

use rayon::prelude::*;


const ITERATIONS: u32 = 128;
const STEP: f64 = 0.002;
const SCAL: u32 = 1;


#[derive(Debug)]
struct Bound {
    x: f64,
    y: f64,
}

impl Bound {
    pub fn new(x: f64, y: f64) -> Bound {
        Bound { x: x, y: y }
    }
}


#[derive(Debug)]
struct FractalPoint {
    is_inside: bool,
    last_value: f64,
    iterations: u32,
}

impl FractalPoint {
    fn mandelbrot(f: Complex64) -> FractalPoint {
        FractalPoint::julia(f, f)
    }

    fn julia(mut f: Complex64, c: Complex64) -> FractalPoint {
        let mut is_inside = true;
        let mut i = 0;

        while i < ITERATIONS {
            f = f * f + c;

            if f.norm() > 2.0 {
                is_inside = false;
                break;
            }

            i += 1;
        }

        FractalPoint {
            last_value: f.norm(),
            iterations: i,
            is_inside: is_inside,
        }
    }

    fn to_pixels(&self) -> Vec<u8> {
        if self.is_inside {
            vec![0, (self.last_value * 128.0) as u8, ((2.0 - self.last_value) * 100.0) as u8]

            //let last_value = (self.last_value * 1_000_000.0) as u32;
            // vec![0, (last_value % 255) as u8, (last_value % 255) as u8]
        } else {
            u32_to_vec(self.iterations)
        }
    }
}


fn main() {
    assert_eq!(FractalPoint::mandelbrot(Complex64::new(0.0, 0.0)).is_inside,
               true);
    assert_eq!(FractalPoint::mandelbrot(Complex64::new(-1.0, 0.0)).is_inside,
               true);
    assert_eq!(FractalPoint::mandelbrot(Complex64::new(1.0, 0.0)).is_inside,
               false);

    let manifest = vec![(Bound::new(-3.0, -1.2), Bound::new(1.0, 1.2), STEP, None),
                        (Bound::new(-3.0, -1.2), Bound::new(2.0, 1.2), STEP, Some(Complex64::new(-0.4, 0.6))),
                        (Bound::new(-3.0, -1.2), Bound::new(2.0, 1.2), STEP, Some(Complex64::new(-0.8, 0.156))),
                        (Bound::new(-1.2, -1.2), Bound::new(1.2, 1.0), STEP, Some(Complex64::new(0.285, 0.01)))];

    for (i, row) in manifest.iter().enumerate() {
        let (ref start, ref end, step, ref c) = *row;

        let frac = {
            if let Some(c) = *c {
                gen_fractal(start, end, step, |f| FractalPoint::julia(f, c))
            } else {
                gen_fractal(start, end, step, FractalPoint::mandelbrot)
            }

        };

        println!("Fractal: {}", i + 1);

        let img = fractal_to_image(&frac);
        let img = img.resize(img.width() * SCAL, img.height() * SCAL, image::CatmullRom);

        let mut fout = &File::create(&format!("{}.png", i + 1)).unwrap();
        img.save(&mut fout, image::PNG).unwrap();
    }
}


fn fractal_to_image(frac: &[Vec<FractalPoint>]) -> image::DynamicImage {
    let width = frac.len();
    let height = frac[0].len();

    // *this is AWESOME*
    let v = (0..height)
        .into_par_iter()
        .flat_map(move |y| (0..width).into_par_iter().flat_map(move |x| frac[x][y].to_pixels()))
        .collect();

    let imgbuf = image::ImageBuffer::from_raw(width as u32, height as u32, v).unwrap();
    image::ImageRgb8(imgbuf).resize_exact(1920, 1080, image::Lanczos3)
}


fn gen_fractal<F>(start: &Bound, end: &Bound, step: f64, gen: F) -> Vec<Vec<FractalPoint>>
    where F: Fn(Complex64) -> FractalPoint
{
    let mut out = vec![];
    let mut x = start.x;

    while x < end.x {
        let mut y = start.y;
        let mut tmp = vec![];

        while y < end.y {
            tmp.push(gen(Complex64::new(x, y)));
            y += step;
        }

        out.push(tmp);
        x += step;
    }

    out
}


fn u32_to_vec(n: u32) -> Vec<u8> {
    vec![(n >> 16) as u8, (n >> 8) as u8, n as u8]
}
