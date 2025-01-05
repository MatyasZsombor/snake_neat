use crate::constants::*;
use crate::entities::*;
use crate::grid_position::*;
use ggez::{event, graphics, Context, GameResult};
use oorandom::Rand32;

pub struct GameState {
    food: Food,
    snake: Snake,
    rng: Rand32,
}

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Failed to create RNG seed!");
        let mut rng = Rand32::new(u64::from_ne_bytes(seed));

        GameState {
            rng,
            food: Food::new(GridPosition::random(&mut rng, GRID_SIZE.0, GRID_SIZE.1)),
            snake: Snake::new(GridPosition::new(GRID_SIZE.0 / 4, GRID_SIZE.1 / 2)),
        }
    }

    fn draw_grid(&mut self, canvas: &mut graphics::Canvas, ctx: &mut Context) -> GameResult {
        for row in 0..GRID_SIZE.1 {
            for col in 0..GRID_SIZE.0 {
                let (x, y) = (
                    col as f32 * SQUARE_SIZE.0 as f32,
                    row as f32 * SQUARE_SIZE.1 as f32,
                );

                let square = graphics::Rect::new(x, y, SQUARE_SIZE.0 as f32, SQUARE_SIZE.1 as f32);

                let border_color = graphics::Color::from_rgb(128, 128, 128);

                let rect_mesh_border = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(1.0),
                    square,
                    border_color,
                )?;
                canvas.draw(&rect_mesh_border, graphics::DrawParam::default());
            }
        }
        Ok(())
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.food.draw(&mut canvas);
        self.snake.draw(&mut canvas);
        self.draw_grid(&mut canvas, ctx)?;

        canvas.finish(ctx)?;
        Ok(())
    }
}
