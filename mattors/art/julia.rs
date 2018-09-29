//! Simple module to draw some [Julia
//! Set](https://en.wikipedia.org/wiki/Julia_set). The most famous one is
//! probably the [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set).

use std::iter::Iterator;

use num::complex::Complex64;

use geo::PointF64;

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
    /// [Julia Set](https://en.wikipedia.org/wiki/Julia_set). `iterations` is
    /// the maximum number of times this function can perform the check to see
    /// whether a given point is inside the set or not.
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
            vec![
                (self.iterations >> 16) as u8,
                (self.iterations >> 8) as u8,
                self.iterations as u8,
            ]
        }
    }
}

/// Iterator that returns all the `FractalPoint`
pub struct JuliaGenIter<F: Fn(Complex64, u32) -> FractalPoint> {
    // params
    start: PointF64,
    xcount: u32,
    ycount: u32,
    stepx: f64,
    stepy: f64,
    iterations: u32,
    gen_fn: F,

    // state
    x: u32,
    y: u32,
}

impl<F: Fn(Complex64, u32) -> FractalPoint> JuliaGenIter<F> {
    /// Create a new `JuliaGenIter` that returns all the `FractalPoint`s from
    /// `start` moving x by `stepx` `xcount` times and y by `stepy` `ycount`
    /// times. Both `ycount` and `xcount` are exclusive. `gen_fn` is the
    /// generator function that takes the current position as a complex number
    /// and that returns the `FractalPoint`.
    pub fn new(
        start: PointF64,
        xcount: u32,
        ycount: u32,
        stepx: f64,
        stepy: f64,
        iterations: u32,
        gen_fn: F,
    ) -> JuliaGenIter<F> {
        JuliaGenIter {
            start,
            xcount,
            ycount,
            stepx,
            stepy,
            iterations,
            gen_fn,
            x: 0,
            y: 0,
        }
    }

    /// Consume the `JuliaGenIter` and return an image of the Julia set formed
    /// by all the points this iterator yields.
    pub fn into_image(self) -> Option<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>> {
        let width = self.xcount;
        let height = self.ycount;

        image::ImageBuffer::from_raw(width, height, self.flat_map(|pt| pt.to_pixels()).collect())
    }
}

impl<F: Fn(Complex64, u32) -> FractalPoint> Iterator for JuliaGenIter<F> {
    type Item = FractalPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.ycount {
            return None;
        }

        let x = self.start.x + f64::from(self.x) * self.stepx;
        let y = self.start.y + f64::from(self.y) * self.stepy;

        let pt = (self.gen_fn)(Complex64::new(x, y), self.iterations);

        self.x += 1;
        if self.x >= self.xcount {
            self.x = 0;
            self.y += 1;
        }

        Some(pt)
    }
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
