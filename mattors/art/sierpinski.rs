//! Generate sierpinski triangle

use std::fmt::Debug;
use std::iter::Iterator;

use rand::Rng;

use geo::PointU32;

use crate::drawing;

/// Handy alias for a [Sierpinski
/// Triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle). Order is top,
/// left and right.
pub type SierpinskiTriangle = (PointU32, PointU32, PointU32);

/// Iterator over all the iterations of the [Sierpinski
/// Triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle)
pub struct SierpinskiIter {
    triangles: Vec<SierpinskiTriangle>,
}

impl SierpinskiIter {
    /// Create a new `SierpinskiIter` bound by the origin and the given `width`
    /// and `height`.
    pub fn new(width: u32, height: u32) -> SierpinskiIter {
        let initial_triangle = (
            PointU32::new(width / 2, 0),
            PointU32::new(0, height - 1),
            PointU32::new(width - 1, height - 1),
        );

        SierpinskiIter {
            triangles: vec![initial_triangle],
        }
    }
}

impl Iterator for SierpinskiIter {
    type Item = Vec<SierpinskiTriangle>;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem::replace;

        let old_triangles = replace(&mut self.triangles, vec![]);

        self.triangles.extend(
            old_triangles
                .iter()
                .flat_map(|&(ref top, ref left, ref right)| {
                    let mid_left =
                        PointU32::new(top.x - (top.x - left.x) / 2, top.y + (left.y - top.y) / 2);
                    let mid_right = PointU32::new(top.x + (top.x - left.x) / 2, mid_left.y);
                    let mid_bottom = PointU32::new(top.x, left.y);

                    let new_top = (*top, mid_left, mid_right);
                    let new_left = (mid_left, *left, mid_bottom);
                    let new_right = (mid_right, mid_bottom, *right);

                    vec![new_top, new_left, new_right].into_iter()
                }),
        );

        Some(old_triangles)
    }
}

/// Draw a fancy [Sierpinski
/// Triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle) on the given
/// image.
pub fn fancy_sierpinski<I>(
    img: &mut I,
    iterations: usize,
    hollow_triangles: bool,
    pixs: &[I::Pixel],
) where
    I: image::GenericImage,
    I::Pixel: Debug,
{
    if pixs.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();

    let (width, height) = img.dimensions();
    let mut siter = SierpinskiIter::new(width, height);

    let mut drawer = drawing::Drawer::new_with_no_blending(img);

    siter
        .next()
        .map(|triangles| {
            drawer.hollow_triangle(
                triangles[0].0,
                triangles[0].1,
                triangles[0].2,
                rng.choose(pixs).unwrap(),
            );

            siter.take(iterations).for_each(|triangles| {
                triangles
                    .iter()
                    .for_each(|&(ref mid_left, ref mid_right, ref mid_bottom)| {
                        let pix = rng.choose(pixs).unwrap();

                        if hollow_triangles {
                            drawer.hollow_triangle(*mid_left, *mid_right, *mid_bottom, pix);
                        } else {
                            drawer.triangle(*mid_left, *mid_right, *mid_bottom, pix);
                        }
                    });
            });
        })
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sierpinski_iter() {
        assert_eq!(
            SierpinskiIter::new(101, 101).take(2).collect::<Vec<_>>(),
            vec![
                vec![(
                    PointU32::new(50, 0),
                    PointU32::new(0, 100),
                    PointU32::new(100, 100),
                )],
                vec![
                    (
                        PointU32::new(50, 0),
                        PointU32::new(25, 50),
                        PointU32::new(75, 50),
                    ),
                    (
                        PointU32::new(25, 50),
                        PointU32::new(0, 100),
                        PointU32::new(50, 100),
                    ),
                    (
                        PointU32::new(75, 50),
                        PointU32::new(50, 100),
                        PointU32::new(100, 100),
                    ),
                ],
            ],
        );
    }
}
