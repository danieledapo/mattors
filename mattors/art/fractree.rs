//! Generate some awesome Fractal Trees.

use std::f64;
use std::fmt::Debug;

use geo::PointU32;

use crate::drawing;

/// Draw a fractal tree onto the given `img` using the given `pix` starting from
/// `pt`. `branching_angle` is the angle to use to draw the branches and
/// `branch_len` is the branch length. `branch_angle_step` is an angle that is
/// added and subtracted from `angle` to move branches. `branch_len_factor` is
/// multiplied with `branch_len` to change the `branch_len`.
#[allow(clippy::too_many_arguments)]
pub fn fractal_tree<I>(
    img: &mut I,
    nbranches: u32,
    pt: PointU32,
    branching_angle: f64,
    branching_angle_step: f64,
    branch_len: f64,
    branch_len_factor: f64,
    pix: &I::Pixel,
) where
    I: image::GenericImage,
    I::Pixel: Debug,
    f64: From<<I::Pixel as image::Pixel>::Subpixel>,
{
    if nbranches == 0 {
        return;
    }

    let breakpoint = {
        let x =
            (<f64 as From<u32>>::from(pt.x) + branching_angle.cos() * branch_len).max(0.0) as u32;
        let y =
            (<f64 as From<u32>>::from(pt.y) + branching_angle.sin() * branch_len).max(0.0) as u32;

        PointU32::new(x, y)
    };

    {
        let mut drawer = drawing::Drawer::new_with_no_blending(img);
        drawer.antialiased_line(pt, breakpoint, pix);
    }

    fractal_tree(
        img,
        nbranches - 1,
        breakpoint,
        branching_angle + branching_angle_step,
        branching_angle_step,
        branch_len * branch_len_factor,
        branch_len_factor,
        pix,
    );

    fractal_tree(
        img,
        nbranches - 1,
        breakpoint,
        branching_angle - branching_angle_step,
        branching_angle_step,
        branch_len * branch_len_factor,
        branch_len_factor,
        pix,
    );
}
