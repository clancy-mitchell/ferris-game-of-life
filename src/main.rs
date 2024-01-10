pub mod board;

use crate::board::{Board, BOARD_HEIGHT, BOARD_WIDTH, CELL_SIZE};
use ggez::{
    event,
    graphics::{self, Color},
    Context, GameResult,
};

const UPDATES_PER_SECOND: u32 = 3;
const SCREEN_WIDTH: f32 = (BOARD_WIDTH as f32) * CELL_SIZE as f32;
const SCREEN_HEIGHT: f32 = (BOARD_HEIGHT as f32) * CELL_SIZE as f32;

struct GameState {
    board: Board,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let mut board = board::Board::new();
        board.randomize_board();
        let s = GameState { board };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(UPDATES_PER_SECOND) {
            self.board.increment_generation();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        self.board.draw(ctx, &mut canvas);
        canvas.finish(ctx)?;
        println!("FPS: {}", ctx.time.fps());
        Ok(())
    }
}
pub fn main() -> GameResult {
    let title: &str = "Ferris' Game of Life";
    let author: &str = "Clancy Mitchell";
    let (ctx, event_loop) = ggez::ContextBuilder::new(&title, &author)
        .window_setup(ggez::conf::WindowSetup::default().title(&title).vsync(true))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let state = GameState::new()?;
    event::run(ctx, event_loop, state)
}
