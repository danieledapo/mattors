extern crate image;
extern crate num;
extern crate rayon;

use std::fs::File;
use std::path::Path;

use num::complex::Complex;

use rayon::prelude::*;


type C64 = Complex<f64>;


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
    last_value: u32,
    iterations: u32,
}

impl FractalPoint {
    fn mandelbrot(f: C64) -> FractalPoint {
        FractalPoint::julia(f, f)
    }

    fn julia(mut f: C64, c: C64) -> FractalPoint {
        let mut is_inside = true;
        let mut i = 0;

        while i < 100 {
            f = f * f + c;

            if f.norm() > 2.0 {
                is_inside = false;
                break;
            }

            i += 1;
        }

        FractalPoint {
            last_value: f.norm() as u32,
            iterations: i,
            is_inside: is_inside,
        }
    }

    fn to_pixels(&self) -> Vec<u8> {
        if self.is_inside {
            vec![0, 128, 0]
        } else {
            vec![(self.iterations >> 16) as u8, (self.iterations >> 8) as u8, self.iterations as u8]
        }
    }
}


fn main() {
    assert_eq!(FractalPoint::mandelbrot(Complex::new(0.0, 0.0)).is_inside,
               true);
    assert_eq!(FractalPoint::mandelbrot(Complex::new(-1.0, 0.0)).is_inside,
               true);
    assert_eq!(FractalPoint::mandelbrot(Complex::new(1.0, 0.0)).is_inside,
               false);

    let step = 0.003;

    let manifest =
        vec![(Bound::new(-3.0, -1.2), Bound::new(1.0, 1.2), step, None),
             (Bound::new(-3.0, -1.2), Bound::new(2.0, 1.2), step, Some(C64::new(-0.4, 0.6))),
             (Bound::new(-3.0, -1.2), Bound::new(2.0, 1.2), step, Some(C64::new(-0.8, 0.156))),
             (Bound::new(-1.2, -1.2), Bound::new(1.2, 1.0), step, Some(C64::new(0.285, 0.01)))];

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

        // print_fractal(&frac);
        fractal_to_image(&format!("{}.png", i + 1), 3, 3, &frac);
    }
}


fn fractal_to_image(path: &str, scalx: u32, scaly: u32, frac: &[Vec<FractalPoint>]) {
    let width = frac.len() as u32 * scalx;
    let height = frac[0].len() as u32 * scaly;

    // *this is AWESOME*
    let v = (0..height)
        .into_par_iter()
        .flat_map(move |y| {
            (0..width).into_par_iter().flat_map(move |x| {
                let x = (x / scalx) as usize;
                let y = (y / scaly) as usize;

                frac[x][y].to_pixels()
            })
        })
        .collect();

    let imgbuf = image::ImageBuffer::from_raw(width, height, v).unwrap();

    let mut fout = &File::create(&Path::new(path)).unwrap();
    image::ImageRgb8(imgbuf).save(&mut fout, image::PNG).unwrap();
}


fn gen_fractal<F>(start: &Bound, end: &Bound, step: f64, gen: F) -> Vec<Vec<FractalPoint>>
    where F: Fn(C64) -> FractalPoint
{
    let mut out = vec![];
    let mut x = start.x;

    while x < end.x {
        let mut y = start.y;
        let mut tmp = vec![];

        while y < end.y {
            tmp.push(gen(Complex::new(x, y)));
            y += step;
        }

        out.push(tmp);
        x += step;
    }

    out
}

// fn print_fractal(frac: &Vec<Vec<FractalPoint>>) {
//     for row in frac {
//         for cell in row {
//             if cell.is_inside {
//                 print!("o");
//             } else {
//                 print!(" ");
//             }
//         }
//         print!("\n");
//     }
// }
