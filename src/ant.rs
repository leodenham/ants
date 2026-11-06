use crate::SimState;
use crate::food::Food;
use crate::nest::Nest;
use crate::pheremone::{PheromoneGrid, PheromoneTile};
use macroquad::color::{BROWN, GREEN, RED};
use macroquad::math::{FloatExt, Vec2};
use macroquad::prelude::{BLUE, draw_rectangle, screen_height};
use macroquad::rand::RandGenerator;
use macroquad::shapes::{DrawRectangleParams, draw_circle, draw_rectangle_ex};
use macroquad::window::screen_width;
use std::cmp::min;
use std::f32::consts::{PI, TAU};

pub struct Ant {
    pos: Vec2,
    direction: f32,
    has_food: bool,
    giving_off_amount: f32,
}

impl Ant {
    const WIDTH: f32 = 3.0;
    const HEIGHT: f32 = 10.0;

    const SPEED: f32 = 150.0; // px / sec

    const RANDOM_ROTATE_RANGE: f32 = TAU / 40.0;
    const EDGE_SHYNESS: f32 = 0.05; // % from distance

    const TURN_HOME_STRENGTH: f32 = 0.005;
    const TURN_FOOD_STRENGTH: f32 = 0.02;

    pub fn new(x: f32, y: f32, dir: f32) -> Self {
        Ant {
            pos: Vec2 { x, y },
            direction: dir,
            has_food: false,
            giving_off_amount: 1.0,
        }
    }

    pub fn step(
        &mut self,
        dt: f32,
        rand: &RandGenerator,
        food: &mut Vec<Food>,
        nest: &mut Nest,
        grid: &PheromoneGrid,
    ) {
        self.pos += Vec2::from_angle(self.direction) * dt * Ant::SPEED;
        self.giving_off_amount *= 0.999;

        // Check food
        if (!self.has_food) {
            match Self::get_food_at(food, self.pos) {
                Some(food) => {
                    self.has_food = true;
                    food.take();
                    self.giving_off_amount = 1.0;
                }
                None => {}
            };
        } else {
            if nest.does_overlap(self.pos) {
                nest.give(rand);
                self.has_food = false;
                self.giving_off_amount = 1.0;
            }
        }

        // Update rotation
        self.direction += rand.gen_range(-Self::RANDOM_ROTATE_RANGE, Self::RANDOM_ROTATE_RANGE);
        self.turn_for_pheromones(grid);
        self.turn_away_from_edges();

        self.direction %= TAU;
    }

    pub fn get_food_at(food: &mut [Food], pos: Vec2) -> Option<&mut Food> {
        food.iter_mut().find(|f| f.does_overlap(pos))
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

    fn get_sample_points(&self) -> Vec<(f32, Vec2)> {
        const NUM_ANGLES: i32 = 5;
        const MAX_ANGLE: f32 = PI;

        (0..NUM_ANGLES)
            .map(|i| (i - NUM_ANGLES / 2) as f32 * MAX_ANGLE / NUM_ANGLES as f32)
            .map(|angle| {
                (
                    angle,
                    Vec2::from_angle(angle + self.direction) * Self::HEIGHT * 1.5 + self.pos,
                )
            })
            .collect()
    }

    fn turn_for_pheromones(&mut self, grid: &PheromoneGrid) {
        let mut dir = 0.0;
        self.get_sample_points().iter().for_each(|(angle, point)| {
            let influence: f32 = match (grid.get_at_point(*point), self.has_food) {
                (None, _) => 0.0,
                (Some(tile), true) => tile.from_home * Self::TURN_HOME_STRENGTH,
                (Some(tile), false) => tile.from_food * Self::TURN_FOOD_STRENGTH,
            };
            dir += influence * angle;
        });
        self.direction += dir / 5.0
    }

    pub fn render(&self) {
        let color = if self.has_food { RED } else { GREEN };
        draw_rectangle_ex(
            self.pos.x - Ant::WIDTH / 2.0,
            self.pos.y - Ant::HEIGHT / 2.0,
            Ant::WIDTH,
            Ant::HEIGHT,
            DrawRectangleParams {
                rotation: self.direction + PI / 2.0,
                color: color,
                offset: Vec2 { x: 0.5, y: 0.5 },
            },
        );

        // self.get_sample_points().iter().for_each(|(_, p)| {
        //     draw_circle(p.x, p.y, 5.0, BROWN);
        // })
    }

    pub fn has_food(&self) -> bool {
        self.has_food
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn giving_off_amount(&self) -> f32 {
        self.giving_off_amount
    }
}
