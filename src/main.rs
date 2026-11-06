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
    selected: Option<usize>,
}

impl SimState {
    const NUM_ANTS: u32 = 100;
    const TILES_WIDTH: usize = 150;
    const TILES_HEIGHT: usize = 150;

    fn new() -> Self {
        let mut ants = Vec::new();

        for i in 0..Self::NUM_ANTS {
            ants.push(Ant::new(
                500.0,
                500.0,
                2.0 * PI * (i as f32) / (Self::NUM_ANTS as f32),
            ));
        }

        let mut food = Vec::new();
        food.push(Food::new(Vec2 { x: 100.0, y: 100.0 }, 5000));

        let nest = Nest::new(500.0, 500.0, 10.0);
        let grid = PheromoneGrid::new(Self::TILES_WIDTH, Self::TILES_HEIGHT);
        SimState {
            ants,
            food,
            nest,
            grid,
            selected: None,
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
            .for_each(|ant| ant.step(dt, &rand, &mut state.food, &mut state.nest, &state.grid));

        state.grid.render();
        state.food.iter().for_each(|food| food.render());
        state.nest.render();
        state.ants.iter().for_each(|ant| ant.render());

        if let Some(selected_ind) = state.selected {
            state.ants[selected_ind].render_selected();
        }

        state.grid.update(dt, &state.ants);

        if is_mouse_button_released(MouseButton::Left) {
            let (x, y) = mouse_position();
            let pos = Vec2::new(x, y);
            let selected = state
                .ants
                .iter()
                .enumerate()
                .min_by(|(i1, ant1), (i2, ant2)| {
                    let dist1 = ant1.pos().distance(pos);
                    let dist2 = ant2.pos().distance(pos);
                    return dist1.total_cmp(&dist2);
                });
            match selected {
                None => {}
                Some((index, _)) => state.selected = Some(index),
            }
        }

        next_frame().await
    }
}
