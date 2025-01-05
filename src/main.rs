mod constants;
mod directions;
mod entities;
mod game_state;
mod grid_position;

use crate::constants::SCREEN_SIZE;
use crate::game_state::GameState;
use ggez::event;

fn main() {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Snake", "Matyas Zsombor")
        .window_setup(ggez::conf::WindowSetup::default().title("SNAKE"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .build()
        .expect("Failed to create a ggez context!");

    let my_game = GameState::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}
