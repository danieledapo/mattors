//! Generate 2d tangled webs inspired by https://inconvergent.net/2019/a-tangle-of-webs/

use std::collections::HashSet;

use rand::Rng;

use geo::point::{PointF64, PointU32};

use crate::drawing::Drawer;

#[derive(Debug, Clone)]
struct Vertex {
    position: PointF64,
    neighbors: HashSet<usize>,
}

/// generate a random image that can vaguely resemble a spider web.
pub fn generate(img: &mut image::RgbImage, iterations: usize, circle_divisions: u8) {
    use std::f64::consts::PI;
    const TWO_PI: f64 = PI * 2.0;

    let mut rng = rand::thread_rng();
    let mut drawer = Drawer::new_with_no_blending(img);

    let (width, height) = drawer.dimensions();
    let width = f64::from(width);
    let height = f64::from(height);
    let scale = width.min(height) * 0.5;

    let mut edges = HashSet::new();
    let mut vertices = vec![Vertex::new(PointF64::new(
        width / 2.0 + scale,
        height / 2.0,
    ))];

    for i in 1..circle_divisions {
        let a = f64::from(i) * TWO_PI / f64::from(circle_divisions - 1);

        let id = usize::from(i);
        let prev_id = id - 1;

        let mut v = Vertex::new(PointF64::new(
            width / 2.0 + a.cos() * scale,
            height / 2.0 + a.sin() * scale,
        ));

        v.neighbors.insert(prev_id);
        vertices.push(v);

        vertices[prev_id].neighbors.insert(id);
        edges.insert((prev_id, id));
    }
    vertices[0]
        .neighbors
        .insert(usize::from(circle_divisions) - 1);
    vertices[usize::from(circle_divisions) - 1]
        .neighbors
        .insert(0);
    edges.insert((vertices.len() - 1, 0));

    for _ in 0..iterations {
        let a0 = rng.gen_range(0.0, TWO_PI);
        let r0 = rng.gen_range(scale / 2.0, scale);
        let p0 = PointF64::new(width / 2.0 + a0.cos() * r0, height / 2.0 + a0.sin() * r0);

        let a1 = rng.gen_range(0.0, TWO_PI);
        let d1 = (width.powi(2) + height.powi(2)).sqrt();
        let p1 = PointF64::new(p0.x + a1.cos() * d1, p0.y + a1.sin() * d1);

        let mut intersections = edges
            .iter()
            .filter_map(|(v0, v1)| {
                let int = segment_intersection(
                    (p0, p1),
                    (vertices[*v0].position, vertices[*v1].position),
                )?;

                Some((int, (*v0, *v1)))
            })
            .collect::<Vec<_>>();

        if intersections.len() < 2 {
            continue;
        }

        intersections.sort_by(|(i1, _), (i2, _)| {
            i1.squared_dist::<f64>(&p0)
                .partial_cmp(&i2.squared_dist(&p0))
                .unwrap()
        });

        for (int, (v0_id, v1_id)) in &intersections[..2] {
            let int = *int;
            let v0_id = *v0_id;
            let v1_id = *v1_id;

            let int_v_id = vertices.len();
            let mut int_v = Vertex::new(int);
            int_v.neighbors.insert(v0_id);
            int_v.neighbors.insert(v1_id);
            vertices.push(int_v);

            let v0 = &mut vertices[v0_id];
            v0.neighbors.remove(&v1_id);
            v0.neighbors.insert(int_v_id);
            edges.remove(&(v0_id, v1_id));
            edges.insert((v0_id, int_v_id));

            let v1 = &mut vertices[v1_id];
            v1.neighbors.remove(&v0_id);
            v1.neighbors.insert(int_v_id);
            edges.remove(&(v1_id, v0_id));
            edges.insert((int_v_id, v1_id));
        }

        let int1_id = vertices.len() - 2;
        let int2_id = vertices.len() - 1;
        vertices[int1_id].neighbors.insert(int2_id);
        vertices[int2_id].neighbors.insert(int1_id);
        edges.insert((vertices.len() - 2, vertices.len() - 1));

        let mut new_vertices = vertices.clone();
        for v in &mut new_vertices {
            for n in &v.neighbors {
                let dx = vertices[*n].position.x - v.position.x;
                let dy = vertices[*n].position.y - v.position.y;
                let l = (dx.powi(2) + dy.powi(2)).sqrt();

                if l < 20.0 {
                    continue;
                }

                v.position.x += dx / l;
                v.position.y += dy / l;
            }
        }
        vertices = new_vertices;
    }

    let line_pt = |p: PointF64| -> PointU32 {
        PointU32::new(
            p.x.max(0.0).min(width) as u32,
            p.y.max(0.0).min(width) as u32,
        )
    };
    for (v0, v1) in &edges {
        drawer.line(
            line_pt(vertices[*v0].position),
            line_pt(vertices[*v1].position),
            &image::Rgb {
                data: [255, 255, 255],
            },
        );
    }
}

impl Vertex {
    fn new(pos: PointF64) -> Self {
        Vertex {
            neighbors: HashSet::new(),
            position: pos,
        }
    }
}

fn segment_intersection(
    (p0, p1): (PointF64, PointF64),
    (q0, q1): (PointF64, PointF64),
) -> Option<PointF64> {
    fn cross(a: PointF64, b: PointF64) -> f64 {
        (a.x * b.y) - (b.x * a.y)
    }

    let sa = PointF64::new(p1.x - p0.x, p1.y - p0.y);
    let sb = PointF64::new(q1.x - q0.x, q1.y - q0.y);
    let u = cross(sa, sb);

    // this is just a safe-guard so we do not divide by zero below.
    // it is not a good way to test for parallel lines
    if u.abs() <= 0.0 {
        return None;
    }

    let ba = PointF64::new(p0.x - q0.x, p0.y - q0.y);
    let ta = cross(sb, ba) / u;
    let tb = cross(sa, ba) / u;

    if ta >= 0.0 && ta <= 1.0 && tb >= 0.0 && tb <= 1.0 {
        Some(PointF64::new(
            p0.x + ta * (p1.x - p0.x),
            p0.y + ta * (p1.y - p0.y),
        ))
    } else {
        None
    }
}
