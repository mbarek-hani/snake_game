mod food;

mod snake;
use food::Food;
use raylib::prelude::*;
use snake::Snake;

pub struct Game {
    snake: Snake,
    food: Food,
    running: bool
}

impl Game {
    pub const CELL_SIZE: f32 = 20.0;
    pub const CELLS_NUM: f32 = 25.0;
    pub const WIDTH: f32 = Self::CELL_SIZE * Self::CELLS_NUM;
    pub const HEIGHT: f32 = Self::CELL_SIZE * Self::CELLS_NUM;
    pub const GREEN: Color = Color::new(173, 204, 96, 255);
    pub const DARK_GREEN: Color = Color::new(43, 51, 24, 255);

    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            snake: Snake::new(),
            food: Food::new(rl, thread, &Snake::new().body),
            running: false,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Self::GREEN);
        self.food.draw(d, Self::CELL_SIZE as i32);
        self.snake.draw(d, Self::CELL_SIZE as i32, Self::DARK_GREEN);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.running = !self.running;
        }
        match self.running {
            true => {
                self.snake.update(rl);
                if self.snake.check_for_wall_collision() || self.snake.check_for_tail_collision() {
                    self.game_over();
                }
                if self.snake.check_for_food_collision(self.food.position()) {
                    self.snake.add_segment = true;
                    self.food.update(&self.snake.body);
                }
            },
            false => {}
        }
        
    }

    pub fn game_over(&mut self) {
        self.food.update(&self.snake.body);
        self.snake = Snake::new();
        self.running = false;
    }

}