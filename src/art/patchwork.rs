//! Some art inspired by the [Patchwork
//! algorithm](https://mattdesl.svbtle.com/pen-plotter-2).

extern crate image;
extern crate rand;

use art::generate_distinct_random_points;
use drawing::Drawer;
use geo::{convex_hull, kmeans, BoundingBox, Point, PointU32};

/// Generate random shapes according to the PatchWork algorithm.
pub fn random_patchwork(img: &mut image::RgbImage, npoints: usize, k: usize) {
    let mut rng = rand::thread_rng();

    let mut points = generate_distinct_random_points(
        &mut rng,
        npoints,
        &BoundingBox::from_dimensions(img.width(), img.height()),
    );

    let mut drawer = Drawer::new_with_no_blending(img);

    loop {
        let clusters = kmeans::kmeans(points.iter().map(|p| p.cast::<i64>()), k, 300);

        let smallest_cluster = clusters
            .iter()
            .filter(|(_, cluster)| cluster.len() > 2)
            .min_by_key(|(_, cluster)| cluster.len());

        match smallest_cluster {
            None => break,
            Some((_pivot, cluster)) => {
                let hull = convex_hull::convex_hull(
                    cluster
                        .iter()
                        .map(|p| Point::new(f64::from(p.x as u32), f64::from(p.y as u32))),
                );

                for pt in cluster {
                    points.remove(&PointU32::new(pt.x as u32, pt.y as u32));
                }

                let pix = image::Rgb {
                    data: [0x50, 0x50, 0x50],
                };

                drawer.closed_path(
                    hull.into_iter()
                        .map(|p| PointU32::new(p.x as u32, p.y as u32)),
                    &pix,
                );

                // TODO: recurse in each polygon performing, regenerate n random
                // points inside that polygon and recalculate everything.
            }
        }
    }
}
