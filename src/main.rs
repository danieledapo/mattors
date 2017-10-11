extern crate image;
extern crate matto;
extern crate num;
extern crate rayon;

use std::fs::File;

use num::complex::Complex64;

use matto::julia::{fractal_to_image, gen_fractal, Bound, FractalPoint};
use matto::dragon;

const STEP: f64 = 0.002;


fn main() {
    julia_fractals();
    spawn_dragons();
}


fn julia_fractals() {
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
        let img = img.resize_exact(1920, 1080, image::Lanczos3);

        let mut fout = &File::create(&format!("{}.png", i + 1)).unwrap();
        img.save(&mut fout, image::PNG).unwrap();
    }
}


fn spawn_dragons() {
    let manifest = [
        (dragon::Move::Left, 1480, 730, &[255, 0, 0]),
        (dragon::Move::Up, 500, 730, &[0, 0, 255]),
    ];

    for (i, row) in manifest.iter().enumerate() {
        println!("Dragon: {}", i + 1);

        let drag = dragon::dragon(17, row.0.clone());
        let img = dragon::dragon_to_image(&drag, 1920, 1080, row.1, row.2, 2, row.3);

        img.save(&format!("dragon-{}.png", i + 1)).unwrap();
    }
}
