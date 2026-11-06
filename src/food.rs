use macroquad::color::{BLUE, PINK};
use macroquad::math::Vec2;
use macroquad::prelude::{Color, draw_circle};
use macroquad::shapes::draw_rectangle;
use std::cell::OnceCell;
use std::f32::consts::PI;

pub struct Food {
    pos: Vec2,
    amount: u32,
    radius: f32,
}

impl Food {
    const AREA_PER_AMOUNT: f32 = 10.0;

    fn calc_radius(amount: u32) -> f32 {
        ((amount as f32) * Self::AREA_PER_AMOUNT / PI).sqrt()
    }

    pub fn new(pos: Vec2, amount: u32) -> Self {
        Food {
            pos,
            amount,
            radius: Self::calc_radius(amount),
        }
    }

    pub fn render(&self) {
        if self.amount == 0 {
            return;
        }

        draw_circle(self.pos.x, self.pos.y, self.radius, BLUE);
    }

    pub fn does_overlap(&self, pos: Vec2) -> bool {
        pos.distance(self.pos) < self.radius
    }

    pub fn take(&mut self) -> bool {
        if self.amount == 0 {
            println!("No food left!");

            return false;
        }
        self.amount -= 1;
        self.radius = Self::calc_radius(self.amount);
        println!("Taking food: {}", self.amount);
        true
    }
}
