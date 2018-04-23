//! Simple module that provides [Image
//! quantization](https://en.wikipedia.org/wiki/Quantization_(image_processing))
//! by implemeting [Median Cut](https://en.wikipedia.org/wiki/Median_cut).

extern crate image;
extern crate num;

use std::collections::HashMap;
use std::convert::From;
use std::hash::Hash;

use self::image::Pixel;

use self::num::cast::NumCast;
use self::num::ToPrimitive;

use utils;

/// Handy type alias to store the occurrence count for a Pixel in a `Vec`.
pub type PixelFreq<P> = (P, u64);

/// Simple structure that contains all the data `quantize` gathered.
#[derive(Debug, PartialEq)]
pub struct QuantizeResult<P>
where
    P: Eq + Hash,
{
    /// the quantized colors
    pub colors: Vec<P>,

    /// from original image pixel to quantized pixel
    pub quantized_pixels: HashMap<P, P>,
}

/// quantize the given sequence of pixels in 2 ^ `divide_steps` colors using
/// [Median Cut](https://en.wikipedia.org/wiki/Median_cut). `None` is something
/// went wrong(i.e. a bug probably).
pub fn quantize<I, P>(pixels: I, divide_steps: u32) -> Option<QuantizeResult<P>>
where
    I: Iterator<Item = P>,
    P: Eq + Hash + Pixel,
    P::Subpixel: Ord,
{
    let pixels_freqs: Vec<PixelFreq<P>> =
        utils::build_hashmap_counter(pixels).into_iter().collect();

    let mut result = Some(QuantizeResult {
        colors: Vec::with_capacity(2_usize.pow(divide_steps)),
        quantized_pixels: HashMap::with_capacity(pixels_freqs.len()),
    });

    quantize_impl(pixels_freqs, divide_steps, &mut result);
    result
}

fn quantize_impl<P>(
    mut pixels_freqs: Vec<PixelFreq<P>>,
    divide_steps: u32,
    mresult: &mut Option<QuantizeResult<P>>,
) where
    P: Eq + Hash + Pixel,
    P::Subpixel: Ord,
{
    if mresult.is_none() {
        return;
    }

    if pixels_freqs.is_empty() {
        return;
    }

    if divide_steps == 0 {
        match get_average_pixel(&pixels_freqs) {
            None => *mresult = None,
            Some(avg_pix) => {
                let res = mresult.as_mut().unwrap();

                res.colors.push(avg_pix);
                for (pix, _) in pixels_freqs {
                    res.quantized_pixels.insert(pix, avg_pix);
                }
            }
        }
        return;
    }

    let mbiggest_chan_range = get_channels_ranges(&pixels_freqs).and_then(|channels_ranges| {
        channels_ranges
            .iter()
            .enumerate()
            .max_by_key(|&(_, &(l, h))| h - l)
            .map(|(i, _)| i)
    });

    if let Some(max_range_chan_idx) = mbiggest_chan_range {
        pixels_freqs.sort_by_key(|p| p.0.channels()[max_range_chan_idx]);

        let (lpixels, rpixels) = pixels_freqs.split_at(pixels_freqs.len() / 2);

        quantize_impl(lpixels.to_vec(), divide_steps - 1, mresult);
        quantize_impl(rpixels.to_vec(), divide_steps - 1, mresult);
    }
}

/// Calculate the pixel obtained as the average among all `pixels_freqs` also
/// considering the frequency each pixel appeared. `None` if `pixels` is empty
/// or there's an error converting between `P::Subpixel` and `usize`.
pub fn get_average_pixel<P>(pixels_freqs: &[PixelFreq<P>]) -> Option<P>
where
    P: Pixel,
{
    if pixels_freqs.is_empty() {
        return None;
    }

    let mut chans_sum = vec![0; From::from(P::channel_count())];
    let mut total_freq = 0;

    for &(pix, freq) in pixels_freqs {
        total_freq += freq;

        for (i, ch) in pix.channels().iter().enumerate() {
            match ch.to_u64() {
                None => return None,
                Some(ch_val) => {
                    chans_sum[i] += ch_val * freq;
                }
            }
        }
    }

    chans_sum
        .iter()
        .map(|ch| NumCast::from(ch / total_freq))
        .collect::<Option<Vec<_>>>()
        .map(|subpixels| *P::from_slice(&subpixels))
}

/// Get the maximum channel range in `pixels` for all the channels. `None` if
/// something went wrong when converting `P::Subpixel` to `u64` or if `pixels`
/// is empty.
pub fn get_channels_ranges<P>(pixels_freqs: &[PixelFreq<P>]) -> Option<Vec<(u64, u64)>>
where
    P: Pixel,
{
    if pixels_freqs.is_empty() {
        return None;
    }

    let mut ranges = vec![(u64::max_value(), u64::min_value()); From::from(P::channel_count())];

    for &(pix, _) in pixels_freqs {
        for (i, ch) in pix.channels().iter().enumerate() {
            match ch.to_u64() {
                None => return None,
                Some(ch_val) => {
                    ranges[i].0 = ranges[i].0.min(ch_val);
                    ranges[i].1 = ranges[i].1.max(ch_val);
                }
            }
        }
    }

    Some(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    use self::image::Rgb;

    #[test]
    fn test_empty_pixels() {
        let pixs: Vec<Rgb<u8>> = vec![];
        let expected = Some(QuantizeResult {
            colors: vec![],
            quantized_pixels: hashmap!{},
        });
        assert_eq!(quantize(pixs.into_iter(), 0), expected);
    }

    #[test]
    fn test_same_color() {
        let black = Rgb { data: [0, 0, 0] };

        let divide_steps = 0;
        let pixs = vec![black, black, black, black, black];
        let expected = Some(QuantizeResult {
            colors: vec![black],
            quantized_pixels: hashmap!{ black => black },
        });

        assert_eq!(quantize(pixs.into_iter(), divide_steps), expected);
    }

    #[test]
    fn test_less_pixels_than_wanted() {
        let black = Rgb { data: [0, 0, 0] };
        let red = Rgb { data: [255, 0, 0] };

        let divide_steps = 42;
        let pixs = vec![black, black, black, black, black, red, red, red];
        let expected = Some(QuantizeResult {
            colors: vec![black, red],
            quantized_pixels: hashmap!{ black => black, red => red },
        });

        assert_eq!(quantize(pixs.into_iter(), divide_steps), expected);
    }

    #[test]
    fn test_50_50() {
        let black = Rgb { data: [0, 0, 0] };
        let red = Rgb { data: [255, 0, 0] };

        let divide_steps = 0;
        let pixs = vec![black, black, red, red];
        let avg_pix = Rgb { data: [127, 0, 0] };

        let expected = Some(QuantizeResult {
            colors: vec![avg_pix],
            quantized_pixels: hashmap!{ black => avg_pix, red => avg_pix },
        });

        assert_eq!(quantize(pixs.into_iter(), divide_steps), expected);
    }

    #[test]
    fn test_different_freqs() {
        let black = Rgb { data: [0, 0, 0] };
        let red = Rgb { data: [255, 0, 0] };

        let divide_steps = 0;
        let pixs = vec![black, black, red, red, red, black, black];
        let avg_pix = Rgb { data: [109, 0, 0] };

        let expected = Some(QuantizeResult {
            colors: vec![avg_pix],
            quantized_pixels: hashmap!{ black => avg_pix, red => avg_pix },
        });

        assert_eq!(quantize(pixs.into_iter(), divide_steps), expected);
    }

    #[test]
    fn test_different_freqs_but_few_colors() {
        let black = Rgb { data: [0, 0, 0] };
        let red = Rgb { data: [255, 0, 0] };

        let divide_steps = 1;
        let pixs = vec![black, black, red, red, red, black, black];
        let expected = Some(QuantizeResult {
            colors: vec![black, red],
            quantized_pixels: hashmap!{ black => black, red => red },
        });

        assert_eq!(quantize(pixs.into_iter(), divide_steps), expected);
    }
}
