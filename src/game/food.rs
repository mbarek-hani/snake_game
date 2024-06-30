use rand::{thread_rng, Rng};
use raylib::prelude::*;
use std::collections::VecDeque;
pub struct Food {
    position: Vector2,
    texture: Texture2D,
}

impl Food {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        snake_body: &VecDeque<Vector2>,
    ) -> Self {
        let image = Image::load_image("./assets/foodv2.png").unwrap();
        let texture = rl.load_texture_from_image(thread, &image).unwrap();
        Self {
            position: Self::generate_random_pos(snake_body),
            texture,
        }
    }

    fn generate_random_pos(snake_body: &VecDeque<Vector2>) -> Vector2 {
        let mut rng = thread_rng();
        let mut x: f32 = rng.gen_range(0..=24) as f32;
        let mut y: f32 = rng.gen_range(0..=24) as f32;
        let mut rnd_pos = Vector2::new(x, y);
        while snake_body.contains(&rnd_pos) {
            x = rng.gen_range(0..=24) as f32;
            y = rng.gen_range(0..=24) as f32;
            rnd_pos = Vector2::new(x, y);
        }
        rnd_pos
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, cell_size: i32, offset: i32) {
        d.draw_texture(
            &self.texture,
            offset + self.position.x as i32 * cell_size,
            offset + self.position.y as i32 * cell_size,
            Color::WHITE,
        );
    }

    pub fn update(&mut self, snake_body: &VecDeque<Vector2>) {
        self.position = Self::generate_random_pos(snake_body);
    }

    pub fn position(&self) -> &Vector2 {
        &self.position
    }
}
