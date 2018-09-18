//! This module contains the code to generate the images

extern crate rand;

use self::rand::Rng;
use std::collections::HashSet;

use geo::{BoundingBox, PointU32};

/// Randomly subdive the given box in at most n subboxes of at least
/// minimum_area.
pub fn random_bbox_subdivisions<R: Rng>(
    n: usize,
    bbox: BoundingBox<u32>,
    minimum_area: u32,
    rng: &mut R,
) -> impl Iterator<Item = BoundingBox<u32>> {
    let mut small_bboxes = vec![];
    let mut boxes = vec![bbox];

    for _ in 0..n {
        if boxes.is_empty() {
            break;
        }

        let i = rng.gen_range(0, boxes.len());
        let b = boxes.swap_remove(i);

        // if either the x or y coordinates are the same then we cannot get a
        // random number because it wouldn't make sense. Just skip the item.
        if b.min().x == b.max().x || b.min().y == b.max().y {
            continue;
        }

        let random_point = random_point_in_bbox(rng, &b);
        let sub_boxes = b.split_at(&random_point).unwrap();

        let mut add_piece = |p: BoundingBox<u32>| {
            if p.area().unwrap() <= minimum_area {
                small_bboxes.push(p);
            } else {
                boxes.push(p);
            }
        };

        add_piece(sub_boxes.0);
        add_piece(sub_boxes.1);
        add_piece(sub_boxes.2);
        add_piece(sub_boxes.3);
    }

    boxes.into_iter().chain(small_bboxes)
}

/// Generate n distinct random points in bbox.
pub fn generate_distinct_random_points<R: Rng>(
    rng: &mut R,
    n: usize,
    bbox: &BoundingBox<u32>,
) -> HashSet<PointU32> {
    let mut points = HashSet::new();

    // TODO: if n > number of points in bbox panic!
    // TODO: if n is high it's probably faster to generate all the points and
    // shuffle the array.
    while points.len() < n {
        points.insert(random_point_in_bbox(rng, bbox));
    }

    points
}

/// Generate a random point in a bbox.
pub fn random_point_in_bbox<R: Rng>(rng: &mut R, bbox: &BoundingBox<u32>) -> PointU32 {
    let x = rng.gen_range(bbox.min().x, bbox.max().x);
    let y = rng.gen_range(bbox.min().y, bbox.max().y);

    PointU32::new(x, y)
}

pub mod delaunay;
pub mod dragon;
pub mod fractree;
pub mod julia;
pub mod mondrian;
pub mod patchwork;
pub mod primi;
pub mod quantize;
pub mod runes;
pub mod sierpinski;
pub mod stippling;
pub mod voronoi;
