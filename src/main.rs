extern crate image;
extern crate num;
extern crate rayon;
extern crate matto;

use std::fs::File;

use image::GenericImage;
use num::complex::Complex64;

use matto::julia::{fractal_to_image, gen_fractal, Bound, FractalPoint};

const STEP: f64 = 0.002;
const SCAL: u32 = 1;


fn main() {
    let manifest = vec![
        (Bound::new(-3.0, -1.2), Bound::new(1.0, 1.2), STEP, None),
        (
            Bound::new(-3.0, -1.2),
            Bound::new(2.0, 1.2),
            STEP,
            Some(Complex64::new(-0.4, 0.6)),
        ),
        (
            Bound::new(-3.0, -1.2),
            Bound::new(2.0, 1.2),
            STEP,
            Some(Complex64::new(-0.8, 0.156)),
        ),
        (
            Bound::new(-1.2, -1.2),
            Bound::new(1.2, 1.0),
            STEP,
            Some(Complex64::new(0.285, 0.01)),
        ),
    ];

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
