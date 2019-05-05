//! Generate some dithered images.

use image::{GrayImage, Luma};

/// Perform [Floyd–Steinberg_dithering][0] over a binary image.
///
/// 0: https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
pub fn dither(img: &GrayImage, mut closest: impl FnMut(&Luma<u8>) -> Luma<u8>) -> GrayImage {
    let mut new = GrayImage::new(img.width(), img.height());

    for y in 0..new.height() {
        for x in 0..new.width() {
            let old_pixel = img.get_pixel(x, y);
            let new_pixel = closest(&old_pixel);
            let err = f32::from(old_pixel[0]) - f32::from(new_pixel[0]);

            let mut distribute_err = |(xx, yy), ratio| {
                if xx >= new.width() || yy >= new.height() {
                    return;
                }

                let p = new.get_pixel_mut(xx, yy);
                *p = Luma {
                    data: [clamp_u8(f32::from(p.data[0]) + err * ratio)],
                };
            };

            distribute_err((x + 1, y), 7.0 / 16.0);

            if x > 0 {
                distribute_err((x - 1, y + 1), 3.0 / 16.0);
            }

            distribute_err((x, y + 1), 5.0 / 16.0);
            distribute_err((x + 1, y + 1), 5.0 / 16.0);

            new.put_pixel(x, y, new_pixel);
        }
    }

    new
}

fn clamp_u8(x: f32) -> u8 {
    x.max(f32::from(u8::min_value()))
        .min(f32::from(u8::max_value())) as u8
}
