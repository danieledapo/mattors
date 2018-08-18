//! Draw some [Voronoi diagrams](https://en.wikipedia.org/wiki/Voronoi_diagram).

extern crate image;
extern crate rand;

use std::collections::HashSet;

use color::{random_color, RandomColorConfig};
use geo::{kdtree, PointU32, Rect};

use self::rand::Rng;

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
        &Rect::new(PointU32::new(0, 0), img.width(), img.height()),
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
    bbox: &Rect<u32>,
) -> HashSet<PointU32> {
    let mut points = HashSet::new();

    // TODO: if n > number of points in bbox panic!
    // TODO: if n is high it's probably faster to generate all the points and
    // shuffle the array.
    while points.len() < n {
        let x = rng.gen_range(bbox.origin.x, bbox.origin.x + bbox.width);
        let y = rng.gen_range(bbox.origin.y, bbox.origin.y + bbox.height);

        points.insert(PointU32::new(x, y));
    }

    points
}
