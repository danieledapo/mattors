//! Simple module to draw some [Dragon
//! Curves](https://en.wikipedia.org/wiki/Dragon_curve).

use geo::PointU32;

use crate::drawing;

/// A move the Dragon Fractal can take
#[derive(Clone, Debug)]
pub enum Move {
    /// Go down
    Down,

    /// Go left
    Left,

    /// Go right
    Right,

    /// Go up
    Up,
}

impl Move {
    /// return the move that is obtained by rotating the current move in
    /// clockwise order.
    pub fn clockwise(&self) -> Move {
        match *self {
            Move::Down => Move::Left,
            Move::Left => Move::Up,
            Move::Right => Move::Down,
            Move::Up => Move::Right,
        }
    }
}

/// A [Dragon Fractal](https://en.wikipedia.org/wiki/Dragon_curve).
#[derive(Debug)]
pub struct Dragon(Vec<Move>);

/// Generate a [Dragon Fractal](https://en.wikipedia.org/wiki/Dragon_curve) from
/// an `initial` move iterating `n` times.
pub fn dragon(n: u32, initial: Move) -> Dragon {
    let mut moves = Vec::with_capacity(2_usize.pow(n));
    moves.push(initial);

    for _ in 0..n {
        let cur_len = moves.len();

        for i in 0..cur_len {
            let mv = moves[cur_len - i - 1].clockwise();
            moves.push(mv);
        }
    }

    Dragon(moves)
}

/// Generate a Fractal(I think) based on the same process as the `dragon`. The
/// difference is that the new move is calculated not from the last move, but
/// the first one.
pub fn horns(n: u32, initial: Move) -> Dragon {
    let mut moves = Vec::with_capacity(2_usize.pow(n));
    moves.push(initial);

    for _ in 0..n {
        let cur_len = moves.len();

        for i in 0..cur_len {
            let mv = moves[i].clockwise();
            moves.push(mv);
        }
    }

    Dragon(moves)
}

/// Generate a [Dragon Fractal](https://en.wikipedia.org/wiki/Dragon_curve) and
/// dump it to an image with the given color.
pub fn dragon_to_image(
    drag: &Dragon,
    width: u32,
    height: u32,
    start_x: u32,
    start_y: u32,
    line_len: u32,
    rgb_color: [u8; 3],
) -> image::RgbImage {
    // TODO: might be interesting to add [perlin
    // noise](https://en.wikipedia.org/wiki/Perlin_noise)
    let mut img = image::ImageBuffer::new(width, height);

    {
        let mut drawer = drawing::Drawer::new_with_no_blending(&mut img);

        let pix = image::Rgb { data: rgb_color };

        let mut x = start_x;
        let mut y = start_y;

        for m in &drag.0 {
            let (nx, ny) = {
                match *m {
                    Move::Down => (x, y.saturating_add(line_len)),
                    Move::Left => (x.saturating_sub(line_len), y),
                    Move::Right => (x.saturating_add(line_len), y),
                    Move::Up => (x, y.saturating_sub(line_len)),
                }
            };

            drawer.line(PointU32::new(x, y), PointU32::new(nx, ny), &pix);

            x = nx;
            y = ny;
        }
    }

    img
}
