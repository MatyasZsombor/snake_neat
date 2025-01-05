use std::collections::VecDeque;

use crate::grid_position::GridPosition;
use ggez::graphics::{self, Canvas};

pub const FOOD_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const HEAD_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BODY_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

pub struct Food {
    pos: GridPosition,
}

impl Food {
    pub fn new(pos: GridPosition) -> Self {
        Self { pos }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.pos.into())
                .color(FOOD_COLOR),
        );
    }
}

pub struct Snake {
    head: GridPosition,
    body: VecDeque<GridPosition>,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Self {
        let mut body = VecDeque::new();
        body.push_back(GridPosition::new(pos.x - 1, pos.y));

        Self { head: pos, body }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for seq in &self.body {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect((*seq).into())
                    .color(BODY_COLOR),
            );
        }

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.head.into())
                .color(HEAD_COLOR),
        );
    }
}
