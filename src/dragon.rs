extern crate image;

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

#[derive(Debug)]
pub struct Dragon(Vec<Move>);

pub fn dragon(n: usize, initial: Move) -> Dragon {
    if n == 0 {
        return Dragon(vec![initial]);
    }

    let little_dragon = dragon(n - 1, initial).0;
    let rotated_dragon = little_dragon
        .iter()
        .by_ref()
        .rev()
        .map(|m| m.clockwise())
        .collect::<Vec<_>>();

    let drag = little_dragon.into_iter().chain(rotated_dragon).collect();

    Dragon(drag)
}

pub fn dragon_to_image(
    drag: &Dragon,
    width: u32,
    height: u32,
    start_x: u32,
    start_y: u32,
    line_len: u32,
    rgb_color: &[u8; 3],
) -> image::RgbImage {
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
