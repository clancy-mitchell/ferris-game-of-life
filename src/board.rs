use array2d::Array2D;
use ggez::graphics::{self, Color, InstanceArray};
use ggez::Context;
use rand::Rng;

pub const BOARD_WIDTH: isize = 32;
pub const BOARD_HEIGHT: isize = 20;
pub const CELL_SIZE: isize = 50;
const CELL_COLOR: Color = Color::new(0.97, 0.30, 0.0, 1.0);
const BORDER_ALIVE: bool = false;

#[derive(Clone)]
struct Life {
    alive: bool,
}
pub struct Board {
    grid: Array2D<Life>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: {
                Array2D::filled_with(
                    Life { alive: false },
                    BOARD_WIDTH as usize,
                    BOARD_HEIGHT as usize,
                )
            },
        }
    }
    pub fn randomize_board(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let state = rng.gen::<bool>();
                self.set_life_state(i, j, state);
            }
        }
    }
    pub fn get_life_state(&self, x: isize, y: isize) -> bool {
        match (x, y) {
            (-1, ..) | (.., -1) => BORDER_ALIVE,
            (BOARD_WIDTH, ..) | (.., BOARD_HEIGHT) => BORDER_ALIVE,
            (_, _) => self.grid[(x as usize, y as usize)].alive,
        }
    }
    pub fn set_life_state(&mut self, x: isize, y: isize, state: bool) {
        self.grid[(x as usize, y as usize)].alive = state;
    }
    pub fn increment_generation(&mut self) {
        let mut next_gen_board = Board::new();
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let state = match self.count_nearby_life(i, j) {
                    2 => self.get_life_state(i, j),
                    3 => true,
                    _ => false,
                };
                next_gen_board.set_life_state(i, j, state);
            }
        }
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let state = next_gen_board.get_life_state(i, j);
                self.set_life_state(i, j, state);
            }
        }
    }
    pub fn count_nearby_life(&self, x: isize, y: isize) -> usize {
        let mut life_count: usize = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                life_count += match self.get_life_state(x + i, y + j) {
                    true => 1,
                    false => 0,
                };
            }
        }
        return life_count;
    }
    pub fn draw(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        let mut instance_array = InstanceArray::new(ctx, None);
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                let color = match self.get_life_state(i, j) {
                    true => CELL_COLOR,
                    false => continue,
                };
                let rect = graphics::Rect {
                    x: i as f32 * CELL_SIZE as f32,
                    y: j as f32 * CELL_SIZE as f32,
                    w: CELL_SIZE as f32,
                    h: CELL_SIZE as f32,
                };
                let cell_drawparam = graphics::DrawParam::new().dest_rect(rect).color(color);
                instance_array.push(cell_drawparam);
            }
        }
        canvas.draw(&instance_array, graphics::DrawParam::new());
    }
}
