mod food;

mod snake;
use food::Food;
use raylib::prelude::*;
use snake::Snake;

pub struct Game {
    snake: Snake,
    food: Food,
    running: bool,
    score: u32,
    eat_sound: Sound<'static>,
    game_over_sound: Sound<'static>,
}

impl Game {
    pub const CELL_SIZE: f32 = 20.0;
    pub const CELLS_NUM: f32 = 25.0;
    pub const OFFSET: f32 = 75.0;
    pub const WIDTH: f32 = Self::OFFSET * 2.0 + Self::CELL_SIZE * Self::CELLS_NUM;
    pub const HEIGHT: f32 = Self::OFFSET * 2.0 + Self::CELL_SIZE * Self::CELLS_NUM;
    pub const GREEN: Color = Color::new(173, 204, 96, 255);
    pub const DARK_GREEN: Color = Color::new(43, 51, 24, 255);

    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        raylib_audio: &'static RaylibAudio,
    ) -> Self {
        let eat_sound = raylib_audio.new_sound("./sounds/eat.mp3").unwrap();
        let game_over_sound = raylib_audio.new_sound("./sounds/wall.mp3").unwrap();
        Self {
            snake: Snake::new(),
            food: Food::new(rl, thread, &Snake::new().body),
            running: false,
            score: 0,
            eat_sound,
            game_over_sound,
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Self::GREEN);
        d.draw_rectangle_lines_ex(
            Rectangle::new(
                Self::OFFSET - 5.0,
                Self::OFFSET - 5.0,
                Self::CELL_SIZE * Self::CELLS_NUM + 10.0,
                Self::CELL_SIZE * Self::CELLS_NUM + 10.0,
            ),
            5.0,
            Self::DARK_GREEN,
        );
        d.draw_text(
            "Snake game!",
            Self::OFFSET as i32 - 5,
            20,
            40,
            Self::DARK_GREEN,
        );
        d.draw_text(
            format!("{}", self.score).as_str(),
            Self::OFFSET as i32 - 5,
            Self::OFFSET as i32 + Self::CELLS_NUM as i32 * Self::CELL_SIZE as i32 + 10,
            40,
            Self::DARK_GREEN,
        );
        if self.running {
            d.draw_text(
                "Press SPACE Key to pause the game.",
                Self::OFFSET as i32 + 100,
                Self::OFFSET as i32 + Self::CELLS_NUM as i32 * Self::CELL_SIZE as i32 + 10,
                15,
                Self::DARK_GREEN,
            );
        } else if self.score == 0 {
            d.draw_text(
                "Press SPACE Key to start the game.",
                Self::OFFSET as i32 + 100,
                Self::OFFSET as i32 + Self::CELLS_NUM as i32 * Self::CELL_SIZE as i32 + 10,
                15,
                Self::DARK_GREEN,
            );
        } else {
            d.draw_text(
                "Press SPACE Key to resume the game.",
                Self::OFFSET as i32 + 100,
                Self::OFFSET as i32 + Self::CELLS_NUM as i32 * Self::CELL_SIZE as i32 + 10,
                15,
                Self::DARK_GREEN,
            );
        }
        self.food
            .draw(d, Self::CELL_SIZE as i32, Self::OFFSET as i32);
        self.snake
            .draw(d, Self::CELL_SIZE as i32, Self::DARK_GREEN, Self::OFFSET);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            self.running = !self.running;
        }
        match self.running {
            true => {
                self.snake.update(rl);
                if self.snake.check_for_wall_collision() || self.snake.check_for_tail_collision() {
                    self.game_over_sound.play();
                    self.game_over();
                }
                if self.snake.check_for_food_collision(self.food.position()) {
                    self.eat_sound.play();
                    self.snake.add_segment = true;
                    self.score += 1;
                    self.food.update(&self.snake.body);
                }
            }
            false => {}
        }
    }

    pub fn game_over(&mut self) {
        self.food.update(&self.snake.body);
        self.snake = Snake::new();
        self.running = false;
        self.score = 0;
    }
}
