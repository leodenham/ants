use macroquad::color::RED;
use macroquad::math::Vec2;
use macroquad::prelude::draw_rectangle;
use macroquad::shapes::{DrawRectangleParams, draw_rectangle_ex};
use std::f32::consts::PI;

pub struct Ant {
    pos: Vec2,
    direction: f32,
}

impl Ant {
    const WIDTH: f32 = 3.0;
    const HEIGHT: f32 = 10.0;

    const SPEED: f32 = 30.0; // px / sec

    pub fn new(x: f32, y: f32, dir: f32) -> Self {
        Ant {
            pos: Vec2 { x, y },
            direction: dir,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.pos += Vec2::from_angle(self.direction) * dt * Ant::SPEED;
    }

    pub fn render(&self) {
        draw_rectangle_ex(
            self.pos.x - Ant::WIDTH / 2.0,
            self.pos.y - Ant::HEIGHT / 2.0,
            Ant::WIDTH,
            Ant::HEIGHT,
            DrawRectangleParams {
                rotation: self.direction + PI / 2.0,
                color: RED,
                offset: Vec2 { x: 0.5, y: 0.5 },
            },
        );
    }
}
