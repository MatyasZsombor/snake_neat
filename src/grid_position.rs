use crate::constants::SQUARE_SIZE;
use ggez::graphics;
use oorandom::Rand32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(rng: &mut Rand32, max_x: i32, max_y: i32) -> Self {
        Self::new(
            rng.rand_range(0..(max_x as u32)) as i32,
            rng.rand_range(0..(max_y as u32)) as i32,
        )
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x * SQUARE_SIZE.0 as i32,
            pos.y * SQUARE_SIZE.1 as i32,
            SQUARE_SIZE.0 as i32,
            SQUARE_SIZE.1 as i32,
        )
    }
}
