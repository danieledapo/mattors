//! Some art inspired by the [Patchwork
//! algorithm](https://mattdesl.svbtle.com/pen-plotter-2).

use std::collections::HashSet;

use geo::{convex_hull, kmeans, BoundingBox, Point, Polygon};

use crate::art::random_point_in_bbox;
use crate::drawing::{Blender, Drawer};

const WHITE_EGG: image::Rgb<u8> = image::Rgb {
    data: [0xFD, 0xFD, 0xFF],
};

const BLACK_MATTERHORN: image::Rgb<u8> = image::Rgb {
    data: [0x52, 0x4B, 0x4B],
};

/// Generate random shapes according to the PatchWork algorithm.
pub fn random_patchwork(
    img: &mut image::RgbImage,
    npoints: usize,
    k: usize,
    iterations: usize,
    fill_polygons: bool,
) {
    let mut generations = vec![vec![Polygon::new(vec![
        Point::new(0.0, 0.0),
        Point::new(f64::from(img.width() - 1), 0.0),
        Point::new(f64::from(img.width() - 1), f64::from(img.height() - 1)),
        Point::new(0.0, f64::from(img.height() - 1)),
    ])
    .unwrap()]];

    let mut drawer = Drawer::new_with_no_blending(img);

    drawer.fill(&WHITE_EGG);

    let mut i = 0;

    while let Some(polygons) = generations.pop() {
        if i >= iterations {
            if fill_polygons {
                for poly in polygons {
                    let poly =
                        Polygon::new(poly.points().into_iter().map(|p| p.try_cast().unwrap()))
                            .unwrap();

                    drawer.polygon(&poly, &BLACK_MATTERHORN);
                }
            }

            break;
        }

        i += 1;

        let new_polygons = polygons
            .into_iter()
            .flat_map(|poly| patchwork_step(&mut drawer, &poly, npoints, k, !fill_polygons))
            .collect::<Vec<_>>();

        if !new_polygons.is_empty() {
            generations.push(new_polygons);
        }
    }
}

fn patchwork_step<B: Blender<image::Rgb<u8>>>(
    drawer: &mut Drawer<image::RgbImage, B>,
    polygon: &Polygon<f64>,
    npoints: usize,
    k: usize,
    draw_polygons_boundary: bool,
) -> Vec<Polygon<f64>> {
    let mut rng = rand::thread_rng();

    let polygon_bbox = BoundingBox::from_points(&[
        polygon.bounding_box().min().try_cast().unwrap(),
        polygon.bounding_box().max().try_cast().unwrap(),
    ]);

    let mut points = (0..npoints)
        .map(|_| random_point_in_bbox(&mut rng, &polygon_bbox))
        .collect::<HashSet<_>>();

    points.retain(|pt| polygon.contains(&pt.cast()));

    if points.len() <= 2 {
        return vec![];
    }

    let mut polygons = vec![];

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
                    cluster.iter().map(|p| p.try_cast::<u32>().unwrap().cast()),
                );

                for pt in cluster {
                    points.remove(&pt.try_cast().unwrap());
                }

                if draw_polygons_boundary {
                    drawer.closed_path(
                        hull.iter().map(|p| p.try_cast().unwrap()),
                        &BLACK_MATTERHORN,
                    );
                }

                if let Some(new_poly) = Polygon::new(hull) {
                    polygons.push(new_poly);
                }
            }
        }
    }

    polygons
}
