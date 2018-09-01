//! This module contains the code to generate the images

extern crate rand;

use self::rand::Rng;
use std::collections::HashSet;

use geo::{BoundingBox, PointU32};

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

pub mod delaunay;
pub mod dragon;
pub mod fractree;
pub mod julia;
pub mod primi;
pub mod quantize;
pub mod runes;
pub mod sierpinski;
pub mod voronoi;
