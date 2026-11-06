use macroquad::color::YELLOW;
use macroquad::math::Vec2;
use macroquad::rand::RandGenerator;
use macroquad::shapes::draw_circle;
use std::f32::consts::PI;

pub struct Nest {
    pos: Vec2,
    radius: f32,
}

impl Nest {
    const DELTA: f32 = 40.0;

    pub fn new(x: f32, y: f32, r: f32) -> Self {
        Nest {
            pos: Vec2::new(x, y),
            radius: r,
        }
    }
    pub fn does_overlap(&self, pos: Vec2) -> bool {
        pos.distance(self.pos) < self.radius
    }

    pub fn give(&mut self, rand: &RandGenerator) {
        // if rand.rand() % 5 != 0 {
        //     return;
        // }

        self.radius = ((PI * self.radius * self.radius + Self::DELTA) / PI).sqrt();
    }

    pub fn render(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, YELLOW);
    }
}
