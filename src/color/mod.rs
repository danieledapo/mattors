//! Simple module that helps with generating good looking colors.

extern crate rand;

use self::rand::Rng;

use geo::line::linear_interpolate;
use geo::point::Point;

/// Simple struct to hold colors in HSV colorspace.
#[derive(Clone, Debug, PartialEq)]
pub struct Hsv((u16, u8, u8));

/// A handy enum that allows customization of random_color by passing a specific
/// hue.
#[derive(Clone, Debug, PartialEq)]
pub enum KnownHue {
    /// Monochrome color
    Monochrome,

    /// Red color
    Red,

    /// Orange color
    Orange,

    /// Yellow color
    Yellow,

    /// Green color
    Green,

    /// Blue color
    Blue,

    /// Purple color
    Purple,

    /// Pink color
    Pink,
}

/// Enum to represent the luminosity of the random color that will be generated.
#[derive(Clone, Debug, PartialEq)]
pub enum Luminosity {
    /// Bright
    Bright,

    /// Light
    Light,

    /// Dark
    Dark,
}

/// The config to configure random_color.
#[derive(Clone, Debug, PartialEq)]
pub struct RandomColorConfig<R> {
    rng: R,

    hue: Option<KnownHue>,
    luminosity: Option<Luminosity>,
}

/// return a new random color that hopefully should be good looking.
/// inspired by https://github.com/davidmerfield/randomColor.
pub fn random_color<R: Rng>(config: &mut RandomColorConfig<R>) -> Hsv {
    let hue = pick_hue(config);
    let saturation = pick_saturation(config, hue);
    let brightness = pick_brightness(hue, saturation);

    Hsv((hue, saturation, brightness))
}

fn pick_hue<R: Rng>(config: &mut RandomColorConfig<R>) -> u16 {
    let range = config
        .hue
        .as_ref()
        .and_then(|hue| hue.hue_range())
        .unwrap_or((0, 360));

    let hue = config.rng.gen_range(range.0, range.1 + 1);

    // Instead of storing red as two seperate ranges,
    // we group them, using negative numbers
    if hue < 0 {
        (hue + 360) as u16
    } else {
        hue as u16
    }
}

fn pick_saturation<R: Rng>(config: &mut RandomColorConfig<R>, hue: u16) -> u8 {
    if let Some(KnownHue::Monochrome) = config.hue.as_ref() {
        return 0;
    }

    let (min_sat, max_sat) = KnownHue::from_hue(hue).saturation_range();

    let sat_range = match config.luminosity.as_ref() {
        None => (min_sat, max_sat),
        Some(Luminosity::Bright) => (55, max_sat),
        Some(Luminosity::Dark) => (max_sat - 10, max_sat),
        Some(Luminosity::Light) => (min_sat, 55),
    };

    config.rng.gen_range(sat_range.0, sat_range.1 + 1)
}

fn pick_brightness(hue: u16, saturation: u8) -> u8 {
    let known_hue = KnownHue::from_hue(hue);
    let lower_bounds = known_hue.lower_bounds();

    for win in lower_bounds.windows(2) {
        let (sat1, bri1) = win[0];
        let (sat2, bri2) = win[1];

        if saturation >= sat1 && saturation <= sat2 {
            return linear_interpolate(
                &Point::new(sat2 as i16, bri1 as i16),
                &Point::new(sat1 as i16, bri2 as i16),
                saturation as i16,
            ).unwrap() as u8;
        }
    }

    0
}

impl Hsv {
    /// Convert a color from Hsv color space to Rgba with the given alpha.
    pub fn to_rgba(&self, alpha: u8) -> [u8; 4] {
        let [r, g, b] = self.to_rgb();
        [r, g, b, alpha]
    }

    /// Convert a color from Hsv color space to Rgb.
    pub fn to_rgb(&self) -> [u8; 3] {
        // HACK: this algorithm doesn't work for h == 0 or h == 1
        let hue = match (self.0).0 {
            0 => 1.0,
            360 => 359.0,
            h => f64::from(h),
        };

        let hue = hue / 360.0;
        let sat = f64::from((self.0).1) / 100.0;
        let bri = f64::from((self.0).2) / 100.0;

        let hue_i = (hue * 6.0).floor();
        let f = hue * 6.0 - hue_i;
        let p = bri * (1.0 - sat);
        let q = bri * (1.0 - f * sat);
        let t = bri * (1.0 - (1.0 - f) * sat);

        let (r, g, b) = match hue_i.round() as u8 {
            0 => (bri, t, p),
            1 => (q, bri, p),
            2 => (p, bri, t),
            3 => (p, q, bri),
            4 => (t, p, bri),
            5 => (bri, p, q),
            _ => panic!("cannot convert hsv to rgb"),
        };

        [
            (r * 255.0).floor() as u8,
            (g * 255.0).floor() as u8,
            (b * 255.0).floor() as u8,
        ]
    }
}

impl RandomColorConfig<rand::ThreadRng> {
    /// Create a new RandomColorConfig
    pub fn new() -> Self {
        RandomColorConfig {
            hue: None,
            luminosity: None,
            rng: rand::thread_rng(),
        }
    }
}

impl<R: Rng> RandomColorConfig<R> {
    /// Set the hue we should generate a random color with.
    pub fn hue(mut self, hue: KnownHue) -> Self {
        self.hue = Some(hue);
        self
    }

    /// Set the luminosity we should generate a random color with.
    pub fn luminosity(mut self, lumi: Luminosity) -> Self {
        self.luminosity = Some(lumi);
        self
    }
}

impl KnownHue {
    /// Return a KnownHue that encloses the given one.
    pub fn from_hue(hue: u16) -> Self {
        // map red to negative values
        let hue = if hue >= 334 && hue <= 360 {
            i32::from(hue) - 360
        } else {
            i32::from(hue)
        };

        let knowns = [
            KnownHue::Red,
            KnownHue::Orange,
            KnownHue::Yellow,
            KnownHue::Green,
            KnownHue::Blue,
            KnownHue::Purple,
            KnownHue::Pink,
        ];

        for known_hue in knowns.into_iter() {
            let (min_sat, max_sat) = known_hue.hue_range().unwrap();

            if min_sat <= hue && max_sat >= hue {
                return known_hue.clone();
            }
        }

        panic!("bug: cannot find a KnownHue for hue {}", hue);
    }

    /// Return the hue range of this KnownHue. Note that Red starts from
    /// negative so that it's easier to work with.
    pub fn hue_range(&self) -> Option<(i32, i32)> {
        match *self {
            KnownHue::Monochrome => None,
            KnownHue::Red => Some((-26, 18)),
            KnownHue::Orange => Some((19, 46)),
            KnownHue::Yellow => Some((47, 62)),
            KnownHue::Green => Some((63, 178)),
            KnownHue::Blue => Some((179, 257)),
            KnownHue::Purple => Some((258, 282)),
            KnownHue::Pink => Some((283, 334)),
        }
    }

    /// Return the saturation range this color can have.
    pub fn saturation_range(&self) -> (u8, u8) {
        let bounds = self.lower_bounds();

        (bounds[0].0, bounds[bounds.len() - 1].0)
    }

    /// Return the brightness range this color can have.
    pub fn brightness_range(&self) -> (u8, u8) {
        let bounds = self.lower_bounds();

        (bounds[0].1, bounds[bounds.len() - 1].1)
    }

    /// Return the valid bounds for saturation and brightness for this color.
    pub fn lower_bounds(&self) -> &[(u8, u8)] {
        match *self {
            KnownHue::Monochrome => &[(0, 0), (100, 0)],
            KnownHue::Red => &[
                (20, 100),
                (30, 92),
                (40, 89),
                (50, 85),
                (60, 78),
                (70, 70),
                (80, 60),
                (90, 55),
                (100, 50),
            ],
            KnownHue::Orange => &[
                (20, 100),
                (30, 93),
                (40, 88),
                (50, 86),
                (60, 85),
                (70, 70),
                (100, 70),
            ],
            KnownHue::Yellow => &[
                (25, 100),
                (40, 94),
                (50, 89),
                (60, 86),
                (70, 84),
                (80, 82),
                (90, 80),
                (100, 75),
            ],
            KnownHue::Green => &[
                (30, 100),
                (40, 90),
                (50, 85),
                (60, 81),
                (70, 74),
                (80, 64),
                (90, 50),
                (100, 40),
            ],
            KnownHue::Blue => &[
                (20, 100),
                (30, 86),
                (40, 80),
                (50, 74),
                (60, 60),
                (70, 52),
                (80, 44),
                (90, 39),
                (100, 35),
            ],
            KnownHue::Purple => &[
                (20, 100),
                (30, 87),
                (40, 79),
                (50, 70),
                (60, 65),
                (70, 59),
                (80, 52),
                (90, 45),
                (100, 42),
            ],
            KnownHue::Pink => &[
                (20, 100),
                (30, 90),
                (40, 86),
                (60, 84),
                (80, 80),
                (90, 75),
                (100, 73),
            ],
        }
    }
}
