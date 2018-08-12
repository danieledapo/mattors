//! Simple module that implements [Delaunay
//! triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use geo::{Point, PointU32, Rect, Triangle};

/// Triangulate the given set of points.
pub fn triangulate(bounding_box: &Rect<u32>, points: HashSet<PointU32>) -> Vec<Triangle<u32>> {
    if points.len() < 3 {
        return vec![];
    }

    let triangles = points
        .into_iter()
        .fold(super_triangles(bounding_box), |triangles, point| {
            add_point(triangles, point.cast())
        });

    // theoretically we should remove the triangles that share vertices with the
    // initial point, but this thing is not for real use.

    // it's safe to cast everything back to u32 since the points started as u32
    // and were not modified
    triangles
        .into_iter()
        .map(|tri| {
            Triangle::new(
                PointU32::new(tri.points[0].x as u32, tri.points[0].y as u32),
                PointU32::new(tri.points[1].x as u32, tri.points[1].y as u32),
                PointU32::new(tri.points[2].x as u32, tri.points[2].y as u32),
            )
        })
        .collect()
}

// the original algorithm works by finding a super triangle that encloses
// all the points, but since we live in a finite space just pickup a random
// point and divide the bounding box in 4 triangles that always cover the
// entire space. It's not acceptable for real triangulation but we're having
// fun here :).
fn super_triangles(bounding_box: &Rect<u32>) -> Vec<Triangle<i64>> {
    let bounds = bounding_box.points();
    let center = bounding_box.center().cast::<i64>();

    (0..bounds.len())
        .map(|i| {
            Triangle::new(
                bounds[i].cast().clone(),
                bounds[(i + 1) % bounds.len()].cast().clone(),
                center.clone(),
            )
        })
        .collect()
}

fn add_point(triangles: Vec<Triangle<i64>>, point: Point<i64>) -> Vec<Triangle<i64>> {
    let mut edges = vec![];
    let mut new_triangles = Vec::with_capacity(triangles.len());

    for triangle in triangles {
        println!("adding point {:?} to triangle {:?}", point, triangle);
        let (circumcenter, radius) = triangle.squared_circumcircle().unwrap();

        if circumcenter.squared_dist::<i64>(&point) <= radius {
            println!("inside triangle's circumcircle");

            edges.push((triangle.points[0].clone(), triangle.points[1].clone()));
            edges.push((triangle.points[1].clone(), triangle.points[2].clone()));
            edges.push((triangle.points[2].clone(), triangle.points[0].clone()));
        } else {
            new_triangles.push(triangle);
        }
    }

    edges = dedup_edges(edges);

    new_triangles.extend(
        edges
            .into_iter()
            .map(|(pt0, pt1)| Triangle::new(pt0, pt1, point.clone())),
    );

    new_triangles
}

fn dedup_edges(edges: Vec<(Point<i64>, Point<i64>)>) -> Vec<(Point<i64>, Point<i64>)> {
    let mut counts = HashMap::with_capacity(edges.len());

    for (start, end) in edges {
        let mut occupied = false;

        let edges = vec![(start.clone(), end.clone()), (end.clone(), start.clone())];

        for edge in edges {
            if let Entry::Occupied(mut o) = counts.entry(edge) {
                // avoid overflow by not incrementing, but by simply putting a
                // value != 1
                *o.get_mut() = 42;
                occupied = true;
                break;
            }
        }

        if !occupied {
            counts.insert((start, end), 1);
        }
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(edge, _count)| edge)
        .collect()
}

#[cfg(test)]
mod test {
    use super::dedup_edges;

    use geo::Point;

    #[test]
    fn test_dedup_edges() {
        let edge1 = (Point::new(42, 12), Point::new(7, 12));
        let redge1 = (edge1.1.clone(), edge1.0.clone());

        let edge2 = (Point::new(42, 73), Point::new(84, 146));
        let redge2 = (edge2.1.clone(), edge2.0.clone());

        let edge3 = (Point::new(23, 32), Point::new(32, 23));

        let edges = vec![
            edge1.clone(),
            edge2.clone(),
            edge1.clone(),
            redge2.clone(),
            edge3.clone(),
            redge1.clone(),
            edge1.clone(),
            redge1.clone(),
        ];

        assert_eq!(dedup_edges(edges), vec![edge3]);
    }
}
