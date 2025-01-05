use std::collections::VecDeque;

use crate::{constants::GRID_SIZE, directions::Direction, grid_position::GridPosition};
use ggez::graphics::{self, Canvas};

pub const FOOD_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const HEAD_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const BODY_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

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
    dir: Direction,
    next_dir: Option<Direction>,
    last_update_dir: Direction,
    pub ate: bool,
}

impl Snake {
    pub fn new(pos: GridPosition) -> Self {
        Self {
            head: pos,
            body: VecDeque::new(),
            dir: Direction::Right,
            next_dir: None,
            last_update_dir: Direction::Right,
            ate: false,
        }
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

    pub fn update(&mut self, food: &Food) -> bool {
        if self.last_update_dir == self.dir && self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            self.next_dir = None;
        }

        self.body.push_front(self.head);
        self.head = GridPosition::new_from_move(self.head, self.dir);

        self.ate = self.head == food.pos;

        if !self.ate {
            self.body.pop_back();
        }

        self.last_update_dir = self.dir;

        self.body.contains(&self.head)
            || self.head.x >= GRID_SIZE.0
            || self.head.x < 0
            || self.head.y >= GRID_SIZE.1
            || self.head.y < 0
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if self.dir != self.last_update_dir && dir.inverse() != self.dir {
            self.next_dir = Some(dir);
        } else if dir.inverse() != self.last_update_dir {
            self.dir = dir;
        }
    }
}
