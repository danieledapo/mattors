//! Generate some triangly art using Delaunay triangulation.

extern crate image;
extern crate rand;

extern crate geo;

use self::rand::Rng;

use self::geo::{delaunay, BoundingBox, PointF64, PointU32};

use color::{random_color, RandomColorConfig};
use drawing;

/// Generate a random triangulation and draws it onto the given image. The
/// points are generated randomly but the image is divided into a grid and each
/// point is contained in a cell.
pub fn random_triangulation<R: Rng>(
    img: &mut image::RgbaImage,
    color_config: &mut RandomColorConfig<R>,
    grid_size: u32,
    alpha: u8,
) {
    let points = random_points_in_grid(img.width(), img.height(), grid_size);

    let triangles = delaunay::triangulate(
        &BoundingBox::from_dimensions(f64::from(img.width()), f64::from(img.height())),
        points,
    );

    {
        let mut drawer = drawing::Drawer::new_with_no_blending(img);

        for triangle in triangles {
            let [ref p1, ref p2, ref p3] = triangle.points;

            let p1 = PointU32::new(p1.x.ceil() as u32, p1.y.ceil() as u32);
            let p2 = PointU32::new(p2.x.ceil() as u32, p2.y.ceil() as u32);
            let p3 = PointU32::new(p3.x.ceil() as u32, p3.y.ceil() as u32);

            let pix = image::Rgba {
                data: random_color(color_config).to_rgba(alpha),
            };

            drawer.triangle(p1, p2, p3, &pix);
        }
    }
}

fn random_points_in_grid(width: u32, height: u32, grid_size: u32) -> Vec<PointF64> {
    let mut rng = rand::thread_rng();

    let square_width = width / grid_size;
    let square_height = height / grid_size;

    let mut out = Vec::with_capacity(grid_size as usize * grid_size as usize);

    for xi in 0..grid_size {
        for yi in 0..grid_size {
            let cur_square_x = f64::from(xi * square_width);
            let cur_square_y = f64::from(yi * square_height);

            let x = rng.gen_range(
                cur_square_x,
                (cur_square_x + f64::from(square_width)).min(f64::from(width)),
            );
            let y = rng.gen_range(
                cur_square_y,
                (cur_square_y + f64::from(square_height)).min(f64::from(height)),
            );

            out.push(PointF64::new(x, y));
        }
    }

    out
}
