//! Simple Rust implementation of
//! [primitive](https://github.com/fogleman/primitive)

extern crate image;
extern crate num;
extern crate rand;

use std;
use std::clone::Clone;
use std::convert::From;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

use self::rand::Rng;

use art::quantize;
use drawing;
use geo;
use utils::clamp;

/// A shape that can be used in the `primitify` function.
pub trait Shape {
    /// Generate a new random `Shape` inside the box.
    fn random(width: u32, height: u32, dx: u32, dy: u32) -> Self;

    /// Create a new version of `Shape` that's slightly changed.
    fn mutate(&self, width: u32, height: u32, dx: u32, dy: u32) -> Self;

    /// Draw the `Shape` onto `dst`.
    fn draw<P>(&self, origin: &PrimifyImage<P>, dst: &mut PrimifyImage<P>)
    where
        P: 'static + image::Pixel + Debug,
        P::Subpixel: From<u8>;

    /// Upscale the shape by the given `factor`.
    fn upscale(&self, factor: u32) -> Self;
}

/// The result of `primitify`.
pub struct Primitized<P: image::Pixel, S: Shape> {
    /// The smallest error the primitization process was able to achieve.
    pub best_error: f64,

    /// The best image the algorithm was able to achieve.
    pub best_image: PrimifyImage<P>,

    /// All the shapes in the image.
    pub shapes: Vec<S>,

    /// The dominant color of the original image.
    pub dominant_color: P,
}

/// Just an handy alias for `ImageBuffer`.
pub type PrimifyImage<P> = image::ImageBuffer<P, Vec<<P as image::Pixel>::Subpixel>>;

/// Simple rust port of [primitive](https://github.com/fogleman/primitive) and
/// primipy. It only supports triangles as of now.
pub fn primify<P, S>(
    img: &PrimifyImage<P>,
    nshapes: usize,
    nmutations: u32,
    dx: u32,
    dy: u32,
) -> Option<Primitized<P, S>>
where
    P: 'static + Eq + Hash + image::Pixel + Debug,
    P::Subpixel: Ord + From<u8> + std::fmt::Debug,
    f64: From<P::Subpixel>,
    S: Shape,
{
    if let Some(dominant) = get_dominant_color(img) {
        let initial_image = image::ImageBuffer::from_pixel(img.width(), img.height(), dominant);

        let mut res = Primitized {
            best_error: get_error(img.iter(), initial_image.iter()),
            shapes: Vec::with_capacity(nshapes),
            best_image: initial_image,
            dominant_color: dominant,
        };

        for _ in 0..nshapes {
            let (new_primified, new_error, shape) =
                generate_shape::<P, S>(img, &res.best_image, nmutations, dx, dy);

            if new_error < res.best_error {
                res.best_error = new_error;
                res.best_image = new_primified;
                res.shapes.push(shape);
            }

            // println!("best_error: {:?}", best_err);
        }

        return Some(res);
    }

    None
}

fn generate_shape<P, S>(
    origin: &PrimifyImage<P>,
    best_primified: &PrimifyImage<P>,
    nmutations: u32,
    dx: u32,
    dy: u32,
) -> (PrimifyImage<P>, f64, S)
where
    P: 'static + image::Pixel + Debug,
    P::Subpixel: From<u8> + std::fmt::Debug,
    f64: From<P::Subpixel>,
    S: Shape,
{
    let mut primified = best_primified.clone();

    let mut shape = S::random(origin.width(), origin.height(), dx, dy);
    shape.draw(origin, &mut primified);

    let mut error = get_error(origin.iter(), primified.iter());

    for _ in 0..nmutations {
        let new_shape = shape.mutate(origin.width(), origin.height(), dx, dy);

        let mut new_primified = best_primified.clone();
        new_shape.draw(origin, &mut new_primified);

        let mut new_error = get_error(origin.iter(), new_primified.iter());

        // println!("error: {:?} new_error: {:?}", error, new_error);

        if new_error < error {
            error = new_error;
            primified = new_primified;
            shape = new_shape;
        }
    }

    (primified, error, shape)
}

fn get_dominant_color<I>(img: &I) -> Option<I::Pixel>
where
    I: image::GenericImageView,
    I::Pixel: Eq + Hash,
    <<I as image::GenericImageView>::Pixel as image::Pixel>::Subpixel: Ord,
{
    let pixels_it = img.pixels().map(|(_, _, p)| p);

    quantize::quantize(pixels_it, 0).map(|res| res.colors[0])
}

fn get_error<'a, I, D>(it1: I, it2: I) -> f64
where
    I: Iterator<Item = &'a D>,
    D: 'a + Clone,
    f64: From<D>,
{
    // root mean square deviation
    it1.zip(it2)
        .map(|(x, y)| (f64::from(x.clone()), f64::from(y.clone())))
        .fold(0.0, |acc, (x, y)| acc + (x - y).powi(2))
        .sqrt()
}

impl Shape for geo::Triangle<u32> {
    fn random(width: u32, height: u32, dx: u32, dy: u32) -> Self {
        let mut rng = rand::thread_rng();

        let dx = i64::from(dx);
        let dy = i64::from(dy);

        let p1 = {
            let x = rng.gen_range(0, width);
            let y = rng.gen_range(0, height);

            geo::PointU32::new(x, y)
        };

        let (p2, p3) = {
            let mut pts = (0..2).map(|_| {
                let x = rng.gen_range(i64::from(p1.x) - dx, i64::from(p1.x) + dx);
                let y = rng.gen_range(i64::from(p1.y) - dy, i64::from(p1.y) + dy);

                geo::PointU32::new(clamp(x, 0, width), clamp(y, 0, height))
            });

            let p2 = pts.next().unwrap();
            let p3 = pts.next().unwrap();

            (p2, p3)
        };

        geo::Triangle::new(p1, p2, p3)
    }

    fn mutate(&self, width: u32, height: u32, dx: u32, dy: u32) -> Self {
        let dx = i64::from(dx);
        let dy = i64::from(dy);

        let mut rng = rand::thread_rng();

        let mut tri = self.clone();
        let pt_ix = rng.gen_range(0, tri.points.len());

        let x = i64::from(tri.points[pt_ix].x) + rng.gen_range(-dx, dx);
        let y = i64::from(tri.points[pt_ix].y) + rng.gen_range(-dy, dy);

        tri.points[pt_ix] = geo::Point::new(clamp(x, 0, width - 1), clamp(y, 0, height - 1));

        tri
    }

    fn draw<P>(&self, origin: &PrimifyImage<P>, dst: &mut PrimifyImage<P>)
    where
        P: 'static + image::Pixel + Debug,
        P::Subpixel: From<u8>,
    {
        let triangle_center = self.centroid();

        // FIXME: take opacity from config
        let pix = origin
            .get_pixel(triangle_center.x, triangle_center.y)
            .map_with_alpha(|c| c, |_| From::from(0x7F));

        let mut drawer = drawing::Drawer::new_with_default_blending(dst);
        drawer.triangle(self.points[0], self.points[1], self.points[2], &pix);
    }

    fn upscale(&self, factor: u32) -> Self {
        geo::Triangle {
            points: [
                geo::Point::new(self.points[0].x * factor, self.points[0].y * factor),
                geo::Point::new(self.points[1].x * factor, self.points[1].y * factor),
                geo::Point::new(self.points[2].x * factor, self.points[2].y * factor),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        assert_eq!(get_error(([] as [u8; 0]).iter(), [].iter()), 0.0);
        assert_eq!(get_error([0_u8].iter(), [2_u8].iter()), 2.0);
        assert_eq!(get_error([3_u8, 1, 3].iter(), [3_u8, 4, 7].iter()), 5.0);

        let err = get_error([3_u8, 1, 3].iter(), [3_u8, 4, 5].iter());

        // round err to two decimal digits to avoid float issues
        let err = (err * 100.0).trunc() / 100.0;
        assert_eq!(err, 3.6);
    }
}
