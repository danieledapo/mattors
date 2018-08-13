//! Simple module that implements [Delaunay
//! triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)

use geo::{Point, PointF64, Rect, Triangle};

/// Triangulate the given set of points. This blows up if degenerate triangles
/// are formed(e.g. completely flat triangles).
pub fn triangulate(bounding_box: &Rect<f64>, points: Vec<PointF64>) -> Vec<Triangle<f64>> {
    if points.len() < 3 {
        return vec![];
    }

    let mut points = points.into_iter();
    let super_triangles = super_triangles(bounding_box, points.next().unwrap());

    let triangles = points.fold(super_triangles, |triangles, point| {
        add_point(triangles, point)
    });

    // theoretically we should remove the triangles that share vertices with the
    // initial point, but this thing is not for real use.

    triangles
}

// the original algorithm works by finding a super triangle that encloses
// all the points, but since we live in a finite space just pickup a random
// point and divide the bounding box in 4 triangles that always cover the
// entire space. It's not acceptable for real triangulation but we're having
// fun here :).
fn super_triangles(bounding_box: &Rect<f64>, first_point: PointF64) -> Vec<Triangle<f64>> {
    let bounds = bounding_box.points();

    (0..bounds.len())
        .map(|i| {
            Triangle::new(
                bounds[i].clone(),
                bounds[(i + 1) % bounds.len()].clone(),
                first_point.clone(),
            )
        })
        .collect()
}

fn add_point(triangles: Vec<Triangle<f64>>, point: Point<f64>) -> Vec<Triangle<f64>> {
    let mut edges = vec![];
    let mut new_triangles = Vec::with_capacity(triangles.len());

    for triangle in triangles {
        let (circumcenter, radius) = triangle.squared_circumcircle().unwrap();

        if circumcenter.squared_dist::<f64>(&point) <= radius {
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

fn dedup_edges(edges: Vec<(Point<f64>, Point<f64>)>) -> Vec<(Point<f64>, Point<f64>)> {
    // super ugly and super inefficient, but we cannot use hashmaps with f64...

    let mut out = vec![];

    for i in 0..edges.len() {
        let mut count = 0;

        for j in 0..edges.len() {
            let (start, end) = &edges[j];
            if edges[i] == (start.clone(), end.clone()) || edges[i] == (end.clone(), start.clone())
            {
                count += 1;
            }
        }

        if count == 1 {
            out.push(edges[i].clone());
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
        let redge1 = (edge1.1.clone(), edge1.0.clone());

        let edge2 = (Point::new(42.0, 73.0), Point::new(84.0, 146.0));
        let redge2 = (edge2.1.clone(), edge2.0.clone());

        let edge3 = (Point::new(23.0, 32.0), Point::new(32.0, 23.0));

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
