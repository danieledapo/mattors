//! Generate sierpinski triangle

extern crate image;

use std::collections::VecDeque;

use drawing;
use point::PointU32;

/// Draw a [Sierpinski
/// Triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle) on the given
/// image.
pub fn sierpinski<I>(img: &mut I, iterations: u32, pix: &I::Pixel)
where
    I: image::GenericImage,
{
    let (width, height) = img.dimensions();

    let initial_triangle = (
        PointU32::new(width / 2, 0),
        PointU32::new(0, height - 1),
        PointU32::new(width - 1, height - 1),
    );
    drawing::hollow_triangle(
        img,
        &initial_triangle.0,
        &initial_triangle.1,
        &initial_triangle.2,
        pix,
    );

    let mut triangles = VecDeque::with_capacity(1);
    triangles.push_back(initial_triangle);

    for _ in 0..iterations {
        triangles = triangles
            .iter()
            .flat_map(|&(ref top, ref left, ref right)| {
                let mid_left =
                    PointU32::new(top.x - (top.x - left.x) / 2, top.y + (left.y - top.y) / 2);
                let mid_right =
                    PointU32::new(top.x + (top.x - left.x) / 2, top.y + (left.y - top.y) / 2);
                let mid_bottom = PointU32::new(top.x, left.y);

                drawing::hollow_triangle(img, &mid_left, &mid_right, &mid_bottom, pix);

                let new_top = (top.clone(), mid_left.clone(), mid_right.clone());
                let new_left = (mid_left.clone(), left.clone(), mid_bottom.clone());
                let new_right = (mid_right.clone(), mid_bottom.clone(), right.clone());

                vec![new_top, new_left, new_right].into_iter()
            })
            .collect();
    }
}
