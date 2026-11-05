mod ant;

use crate::ant::Ant;
use macroquad::prelude::*;
use macroquad::rand::RandGenerator;
use std::f32::consts::PI;
use std::time::Instant;

struct SimState {
    ants: Vec<Ant>,
}

impl SimState {
    const NUM_ANTS: u32 = 100;
    fn new() -> Self {
        let mut ants = Vec::new();

        for i in 0..Self::NUM_ANTS {
            ants.push(Ant::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
                2.0 * PI * (i as f32) / (Self::NUM_ANTS as f32),
            ));
        }

        SimState { ants }
    }
}

#[macroquad::main("Ants")]
async fn main() {
    let mut state = SimState::new();
    let rand = RandGenerator::new();

    loop {
        clear_background(BLACK);
        let frame_time = get_frame_time();
        state
            .ants
            .iter_mut()
            .for_each(|ant| ant.step(frame_time, &rand));

        state.ants.iter().for_each(|ant| ant.render());

        next_frame().await
    }
}
