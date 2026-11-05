use macroquad::color::RED;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_rectangle, screen_height};
use macroquad::rand::RandGenerator;
use macroquad::shapes::{DrawRectangleParams, draw_rectangle_ex};
use macroquad::window::screen_width;
use std::cmp::min;
use std::f32::consts::{PI, TAU};

pub struct Ant {
    pos: Vec2,
    direction: f32,
}

impl Ant {
    const WIDTH: f32 = 3.0;
    const HEIGHT: f32 = 10.0;

    const SPEED: f32 = 150.0; // px / sec

    const RANDOM_ROTATE_RANGE: f32 = TAU / 40.0;
    const EDGE_SHYNESS: f32 = 0.05; // % from distance

    pub fn new(x: f32, y: f32, dir: f32) -> Self {
        Ant {
            pos: Vec2 { x, y },
            direction: dir,
        }
    }

    pub fn step(&mut self, dt: f32, rand: &RandGenerator) {
        self.pos += Vec2::from_angle(self.direction) * dt * Ant::SPEED;

        // Update rotation
        self.direction += rand.gen_range(-Self::RANDOM_ROTATE_RANGE, Self::RANDOM_ROTATE_RANGE);
        self.turn_away_from_edges();

        self.direction %= TAU;
    }

    fn turn_away_from_edges(&mut self) {
        let dist_from_side = screen_width() * Self::EDGE_SHYNESS;
        let width = screen_width();
        if self.pos.x <= dist_from_side || self.pos.x >= width - dist_from_side {
            // 0 when at edge, 1 when just entered
            let dist_in_shy_area = self.pos.x.min(width - self.pos.x) / (dist_from_side);

            // Getting close to edge, try slow down x direction
            let mut dir_vec = Vec2::from_angle(self.direction);

            dir_vec.x *= dist_in_shy_area;
            if (dir_vec.x < 0.3 && dir_vec.x > -0.3) {
                if (self.pos.x <= dist_from_side) {
                    dir_vec.x += 0.1;
                } else {
                    dir_vec.x -= 0.1;
                }
            }
            self.direction = dir_vec.to_angle();
        }

        let dist_from_top = screen_height() * Self::EDGE_SHYNESS;
        let height = screen_height();

        if self.pos.y <= dist_from_top || self.pos.y >= height - dist_from_top {
            // 0 when at edge, 1 when just entered
            let dist_in_shy_area = (self.pos.y.min(height - self.pos.y) / dist_from_top);

            let mut dir_vec = Vec2::from_angle(self.direction);
            dir_vec.y *= dist_in_shy_area;
            if (dir_vec.y < 0.3 && dir_vec.y > -0.3) {
                if (self.pos.y <= dist_from_top) {
                    dir_vec.y += 0.1;
                } else {
                    dir_vec.y -= 0.1;
                }
            }
            self.direction = dir_vec.to_angle();
        }
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
