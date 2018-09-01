//! Draw some [Voronoi diagrams](https://en.wikipedia.org/wiki/Voronoi_diagram).

extern crate image;
extern crate rand;

use std::collections::HashSet;

use color::{random_color, RandomColorConfig};
use geo::{kdtree, BoundingBox, PointU32};

use self::rand::Rng;

/// Generate a voronoi diagram where the colors are taken from the gradient
/// going from color1 to color2.
pub fn gradient_voronoi(
    img: &mut image::RgbImage,
    color1: image::Rgb<u8>,
    color2: image::Rgb<u8>,
    npoints: usize,
) {
    if npoints == 0 {
        return;
    }

    let random_points = generate_distinct_random_points(
        &mut rand::thread_rng(),
        npoints,
        &BoundingBox::from_dimensions(img.width(), img.height()),
    );

    let points = random_points.iter().map(|pt| (*pt, ())).collect();
    let points = kdtree::KdTree::from_vector(points);

    let dr = f64::from(color2[0]) - f64::from(color1[0]);
    let dg = f64::from(color2[1]) - f64::from(color1[1]);
    let db = f64::from(color2[2]) - f64::from(color1[2]);

    let img_width = img.width();

    for (x, y, pix) in img.enumerate_pixels_mut() {
        let (closest_point, _) = points.nearest_neighbor(PointU32::new(x, y)).unwrap();

        let c = f64::from(closest_point.x) / f64::from(img_width);
        *pix = image::Rgb {
            data: [
                (f64::from(color1[0]) + c * dr) as u8,
                (f64::from(color1[1]) + c * dg) as u8,
                (f64::from(color1[2]) + c * db) as u8,
            ],
        };
    }
}

/// Generate some random Voronoi diagrams.
pub fn random_voronoi<R: Rng>(
    img: &mut image::RgbImage,
    color_config: &mut RandomColorConfig<R>,
    npoints: usize,
) {
    if npoints == 0 {
        return;
    }

    let random_points = generate_distinct_random_points(
        &mut rand::thread_rng(),
        npoints,
        &BoundingBox::from_dimensions(img.width(), img.height()),
    );

    let points = random_points
        .iter()
        .map(|pt| {
            (
                *pt,
                image::Rgb {
                    data: random_color(color_config).to_rgb(),
                },
            )
        })
        .collect();

    let points = kdtree::KdTree::from_vector(points);

    for (x, y, pix) in img.enumerate_pixels_mut() {
        let (_, closest_point_color) = points.nearest_neighbor(PointU32::new(x, y)).unwrap();

        *pix = *closest_point_color;
    }

    // for point in random_points {
    //     img.put_pixel(point.x, point.y, image::Rgb { data: [0, 0, 0] });
    // }
}

fn generate_distinct_random_points<R: Rng>(
    rng: &mut R,
    n: usize,
    bbox: &BoundingBox<u32>,
) -> HashSet<PointU32> {
    let mut points = HashSet::new();

    // TODO: if n > number of points in bbox panic!
    // TODO: if n is high it's probably faster to generate all the points and
    // shuffle the array.
    while points.len() < n {
        let x = rng.gen_range(bbox.min().x, bbox.max().x);
        let y = rng.gen_range(bbox.min().y, bbox.max().y);

        points.insert(PointU32::new(x, y));
    }

    points
}
