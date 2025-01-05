use crate::constants::SQUARE_SIZE;
use crate::directions::Direction;
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

    pub fn new_from_move(pos: GridPosition, dir: Direction) -> Self {
        match dir {
            Direction::Up => GridPosition::new(pos.x, pos.y - 1),
            Direction::Down => GridPosition::new(pos.x, pos.y + 1),
            Direction::Left => GridPosition::new(pos.x - 1, pos.y),
            Direction::Right => GridPosition::new(pos.x + 1, pos.y),
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x * SQUARE_SIZE.0,
            pos.y * SQUARE_SIZE.1,
            SQUARE_SIZE.0,
            SQUARE_SIZE.1,
        )
    }
}
