extern crate image;
extern crate num;

use self::num::complex::Complex64;

use point::Point;

/// This struct is mainly used to pass some data used when converting to raw
/// pixels.
#[derive(Debug)]
pub struct FractalPoint {
    is_inside: bool,
    last_value: f64,
    iterations: u32,
}

impl FractalPoint {
    /// Calculate if the given `f`(that is point) is in the [Mandelbrot
    /// Set](https://en.wikipedia.org/wiki/Mandelbrot_set).
    pub fn mandelbrot(f: Complex64, iterations: u32) -> FractalPoint {
        FractalPoint::julia(f, f, iterations)
    }

    /// Calculate if the given `f`(that is point) with param `c` is in the
    /// [Julia Set](https://en.wikipedia.org/wiki/Julia_set).
    pub fn julia(mut f: Complex64, c: Complex64, iterations: u32) -> FractalPoint {
        let mut is_inside = true;
        let mut i = 0;

        while i < iterations {
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
            is_inside,
        }
    }

    fn to_pixels(&self) -> Vec<u8> {
        if self.is_inside {
            vec![
                0,
                (self.last_value * 128.0) as u8,
                ((2.0 - self.last_value) * 100.0) as u8,
            ]

        //let last_value = (self.last_value * 1_000_000.0) as u32;
        // vec![0, (last_value % 255) as u8, (last_value % 255) as u8]
        } else {
            u32_to_vec(self.iterations)
        }
    }
}

/// Generate a fractal starting from the given `point` and incrementing x by
/// `xstep` and y by `ystep` for `xcount` and `ycount` respectively.
/// `iterations` is the number of iterations `gen` functions should take before
/// saying the point is outside the fractal. `gen` is the generator function
/// that takes the current position as a complex number and that returns the
/// `FractalPoint`.
pub fn gen_fractal<F>(
    start: &Point,
    xcount: u32,
    ycount: u32,
    stepx: f64,
    stepy: f64,
    iterations: u32,
    gen: F,
) -> Vec<Vec<FractalPoint>>
where
    F: Sync + Send + Fn(Complex64, u32) -> FractalPoint,
{
    (0..xcount)
        .map(|ix| {
            (0..ycount)
                .map(|iy| {
                    let x = start.x + f64::from(ix) * stepx;
                    let y = start.y + f64::from(iy) * stepy;

                    gen(Complex64::new(x, y), iterations)
                })
                .collect()
        })
        .collect()
}

/// Create an image from the given fractal.
pub fn fractal_to_image(frac: &[Vec<FractalPoint>]) -> image::DynamicImage {
    let width = frac.len();
    let height = frac[0].len();

    let v = (0..height)
        .flat_map(move |y| (0..width).flat_map(move |x| frac[x][y].to_pixels()))
        .collect();

    let imgbuf = image::ImageBuffer::from_raw(width as u32, height as u32, v).unwrap();
    image::ImageRgb8(imgbuf)
}

fn u32_to_vec(n: u32) -> Vec<u8> {
    vec![(n >> 16) as u8, (n >> 8) as u8, n as u8]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(
            FractalPoint::mandelbrot(Complex64::new(0.0, 0.0), 128).is_inside,
            true
        );
        assert_eq!(
            FractalPoint::mandelbrot(Complex64::new(-1.0, 0.0), 64).is_inside,
            true
        );
        assert_eq!(
            FractalPoint::mandelbrot(Complex64::new(1.0, 0.0), 12).is_inside,
            false
        );
    }
}
