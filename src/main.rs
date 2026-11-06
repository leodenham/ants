mod ant;
mod food;
mod nest;
mod pheremone;

use crate::ant::Ant;
use crate::food::Food;
use crate::nest::Nest;
use crate::pheremone::PheromoneGrid;
use macroquad::prelude::*;
use macroquad::rand::RandGenerator;
use std::f32::consts::PI;
use std::time::Instant;

pub struct SimState {
    ants: Vec<Ant>,
    food: Vec<Food>,
    nest: Nest,
    grid: PheromoneGrid,
}

impl SimState {
    const NUM_ANTS: u32 = 1000;
    const TILES_WIDTH: usize = 150;
    const TILES_HEIGHT: usize = 150;

    fn new() -> Self {
        let mut ants = Vec::new();

        for i in 0..Self::NUM_ANTS {
            ants.push(Ant::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
                2.0 * PI * (i as f32) / (Self::NUM_ANTS as f32),
            ));
        }

        let mut food = Vec::new();
        food.push(Food::new(Vec2 { x: 100.0, y: 100.0 }, 50));

        let nest = Nest::new(500.0, 500.0, 10.0);
        let grid = PheromoneGrid::new(Self::TILES_WIDTH, Self::TILES_HEIGHT);
        SimState {
            ants,
            food,
            nest,
            grid,
        }
    }
}

#[macroquad::main("Ants")]
async fn main() {
    let mut state = SimState::new();
    let rand = RandGenerator::new();

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        state
            .ants
            .iter_mut()
            .for_each(|ant| ant.step(dt, &rand, &mut state.food, &mut state.nest));

        state.grid.render();
        state.food.iter().for_each(|food| food.render());
        state.nest.render();
        state.ants.iter().for_each(|ant| ant.render());

        state.grid.update(dt, &state.ants);

        next_frame().await
    }
}
