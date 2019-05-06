//! Generate some dithered images.

use image::{GenericImageView, ImageBuffer, Pixel};
use num::traits::{AsPrimitive, Bounded};

/// Perform [Floyd–Steinberg_dithering][0] over a binary image.
///
/// 0: https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
pub fn dither<I: GenericImageView>(
    img: &I,
    mut closest: impl FnMut(&I::Pixel) -> I::Pixel,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I: GenericImageView,
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
    f32: From<<I::Pixel as Pixel>::Subpixel> + AsPrimitive<<I::Pixel as Pixel>::Subpixel>,
{
    let min_value = <I::Pixel as Pixel>::Subpixel::min_value();
    let max_value = <I::Pixel as Pixel>::Subpixel::max_value();

    let mut new: ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>> =
        ImageBuffer::new(img.width(), img.height());

    for y in 0..new.height() {
        for x in 0..new.width() {
            let old_pixel = img.get_pixel(x, y);
            let new_pixel = closest(&old_pixel);

            let mut err = [0.0; 3];
            for (e, (o, n)) in err
                .iter_mut()
                .zip(old_pixel.channels().iter().zip(new_pixel.channels()))
            {
                *e = f32::from(*o) - f32::from(*n);
            }

            let mut distribute_err = |(xx, yy), ratio| {
                if xx >= new.width() || yy >= new.height() {
                    return;
                }

                let p = new.get_pixel_mut(xx, yy);
                for (sp, e) in p.channels_mut().iter_mut().zip(&err) {
                    let nsp: f32 = f32::from(*sp) + e * ratio;

                    *sp = nsp
                        .max(f32::from(min_value))
                        .min(f32::from(max_value))
                        .as_();
                }
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
