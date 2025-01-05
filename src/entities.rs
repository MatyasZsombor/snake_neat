use crate::constants::FOOD_COLOR;
use crate::grid_position::GridPosition;
use ggez::graphics;

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
