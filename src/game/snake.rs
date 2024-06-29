use raylib::core::math::Vector2;
use raylib::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT
}
pub struct Snake {
    pub body: VecDeque<Vector2>,
    next_direction: Direction,
    current_direction: Direction,
    pub add_segment: bool
}

impl Snake {
    pub fn new() -> Self{
        Self {
            body: VecDeque::from([Vector2::new(6.0,9.0), Vector2::new(5.0,9.0), Vector2::new(4.0,9.0)]),
            next_direction: Direction::RIGHT,
            current_direction: Direction::RIGHT,
            add_segment: false
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, cell_size: i32, color: Color) {
        for tail in &self.body {
            let rec = Rectangle::new(tail.x * cell_size as f32, tail.y * cell_size as f32, cell_size as f32, cell_size as f32);
            d.draw_rectangle_rounded(rec, 0.7, 6, color);
        }
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            if self.current_direction != Direction::DOWN {
                self.next_direction = Direction::UP;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            if self.current_direction != Direction::UP {
                self.next_direction = Direction::DOWN;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            if self.current_direction != Direction::LEFT {
                self.next_direction = Direction::RIGHT;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            if self.current_direction != Direction::RIGHT {
                self.next_direction = Direction::LEFT;
            }
        }

        let current_head = self.body.get(0).unwrap();
        match self.next_direction {
            Direction::UP => {
                self.body.push_front(*current_head + Vector2::new(0.0, -1.0));
                self.current_direction = Direction::UP;
            },
            Direction::DOWN => {
                self.body.push_front(*current_head + Vector2::new(0.0, 1.0));
                self.current_direction = Direction::DOWN;
            },
            Direction::RIGHT => {
                self.body.push_front(*current_head + Vector2::new(1.0, 0.0));
                self.current_direction = Direction::RIGHT;
            },
            Direction::LEFT => {
                self.body.push_front(*current_head + Vector2::new(-1.0, 0.0));
                self.current_direction = Direction::LEFT;
            }
        }
        if !self.add_segment{ // check if we want to add a segment to the snake body or not
            self.body.pop_back();// if we don't we pop the last segment and thene add a segmet to the head
        } else {
            self.add_segment = false;
        }
    }
    pub fn check_for_wall_collision(&self) -> bool {
        let head = self.body.get(0).unwrap();
        if head.x < 0.0 || head.x > 24.0 || head.y < 0.0 || head.y > 24.0 {
            return true;
        }
        return false;
    }
    pub fn check_for_food_collision(&self, position: &Vector2) -> bool {
        let head = self.body.get(0).unwrap();
        if head == position {
            return true;
        }
        return false;
    }
    pub fn check_for_tail_collision(&self) -> bool {
        let mut body_copy = self.body.clone();
        let head = body_copy.pop_front().unwrap();
        if body_copy.contains(&head){
            return true;
        }
        return false;
    }

}