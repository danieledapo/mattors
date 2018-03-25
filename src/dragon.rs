extern crate image;

/// A move the Dragon Fractal can take
#[derive(Clone, Debug)]
pub enum Move {
    Down,
    Left,
    Right,
    Up,
}

impl Move {
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
    rgb_color: &[u8; 3],
) -> image::RgbImage {
    // TODO: might be interesting to add [perlin
    // noise](https://en.wikipedia.org/wiki/Perlin_noise)
    let mut img = image::ImageBuffer::new(width, height);

    // turn u32 to i64 because we can go < 0
    let line_len = i64::from(line_len);
    let mut x = i64::from(start_x);
    let mut y = i64::from(start_y);

    for m in &drag.0 {
        let (dx, dy) = {
            match *m {
                Move::Down => (0, 1),
                Move::Left => (-1, 0),
                Move::Right => (1, 0),
                Move::Up => (0, -1),
            }
        };

        for _ in 0..line_len {
            x += dx;
            y += dy;

            if x < 0 || x >= (i64::from(width)) {
                continue;
            }

            if y < 0 || y >= (i64::from(height)) {
                continue;
            }

            let pix = image::Rgb(*rgb_color);
            img.put_pixel(x as u32, y as u32, pix);
        }
    }

    img
}
