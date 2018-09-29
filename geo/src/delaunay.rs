//! Simple module that implements [Delaunay
//! triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)

use crate::bbox::BoundingBox;
use crate::point::{Point, PointF64};
use crate::triangle::Triangle;

/// Triangulate the given set of points. This blows up if degenerate triangles
/// are formed(e.g. completely flat triangles).
pub fn triangulate(bounding_box: &BoundingBox<f64>, points: Vec<PointF64>) -> Vec<Triangle<f64>> {
    if points.len() < 3 {
        return vec![];
    }

    let mut points = points.into_iter();
    let super_triangles = super_triangles(bounding_box, &points.next().unwrap());

    // theoretically we should remove the triangles that share vertices with the
    // initial point, but this thing is not for real use.

    points.fold(super_triangles, |triangles, point| {
        add_point(triangles, &point)
    })
}

// the original algorithm works by finding a super triangle that encloses
// all the points, but since we live in a finite space just pickup a random
// point and divide the bounding box in 4 triangles that always cover the
// entire space. It's not acceptable for real triangulation but we're having
// fun here :).
fn super_triangles(bounding_box: &BoundingBox<f64>, first_point: &PointF64) -> Vec<Triangle<f64>> {
    let bounds = bounding_box.points();

    (0..bounds.len())
        .map(|i| Triangle::new(bounds[i], bounds[(i + 1) % bounds.len()], *first_point))
        .collect()
}

fn add_point(triangles: Vec<Triangle<f64>>, point: &Point<f64>) -> Vec<Triangle<f64>> {
    let mut edges = vec![];
    let mut new_triangles = Vec::with_capacity(triangles.len());

    for triangle in triangles {
        let (circumcenter, radius) = triangle.squared_circumcircle().unwrap();

        if circumcenter.squared_dist::<f64>(point) <= radius {
            edges.push((triangle.points[0], triangle.points[1]));
            edges.push((triangle.points[1], triangle.points[2]));
            edges.push((triangle.points[2], triangle.points[0]));
        } else {
            new_triangles.push(triangle);
        }
    }

    edges = dedup_edges(&edges);

    new_triangles.extend(
        edges
            .into_iter()
            .map(|(pt0, pt1)| Triangle::new(pt0, pt1, *point)),
    );

    new_triangles
}

fn dedup_edges(edges: &[(Point<f64>, Point<f64>)]) -> Vec<(Point<f64>, Point<f64>)> {
    // super ugly and super inefficient, but we cannot use hashmaps with f64...

    let mut out = vec![];

    for i in 0..edges.len() {
        let mut count = 0;

        for j in 0..edges.len() {
            let (start, end) = &edges[j];
            if edges[i] == (*start, *end) || edges[i] == (*end, *start) {
                count += 1;
            }
        }

        if count == 1 {
            out.push(edges[i]);
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::dedup_edges;

    use geo::Point;

    #[test]
    fn test_dedup_edges() {
        let edge1 = (Point::new(42.0, 12.0), Point::new(7.0, 12.0));
        let redge1 = (edge1.1, edge1.0);

        let edge2 = (Point::new(42.0, 73.0), Point::new(84.0, 146.0));
        let redge2 = (edge2.1, edge2.0);

        let edge3 = (Point::new(23.0, 32.0), Point::new(32.0, 23.0));

        let edges = vec![edge1, edge2, edge1, redge2, edge3, redge1, edge1, redge1];

        assert_eq!(dedup_edges(&edges), vec![edge3]);
    }
}
