use crate::ant::Ant;
use macroquad::color::RED;
use macroquad::math::Vec2;
use macroquad::prelude::{Color, screen_width};
use macroquad::shapes::draw_rectangle;
use macroquad::window::screen_height;

#[derive(Clone)]
pub struct PheromoneTile {
    pub from_food: f32,
    pub from_home: f32,
}

pub struct PheromoneGrid {
    width: usize,
    height: usize,
    tiles: Vec<PheromoneTile>,
}

impl PheromoneTile {
    pub fn new() -> Self {
        PheromoneTile {
            from_home: 0.0,
            from_food: 0.0,
        }
    }
}

impl PheromoneGrid {
    // dimensions in num tiles
    pub fn new(width: usize, height: usize) -> Self {
        PheromoneGrid {
            width,
            height,
            tiles: vec![PheromoneTile::new(); width * height],
        }
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    fn get_tile_sizes(&self) -> (f32, f32) {
        (
            screen_width() / self.width as f32,
            screen_height() / self.height as f32,
        )
    }

    pub fn render(&self) {
        let (tile_width, tile_height) = self.get_tile_sizes();

        self.tiles.iter().enumerate().for_each(|(index, tile)| {
            let (i, j) = self.get_pos(index);
            draw_rectangle(
                (i as f32) * tile_width,
                (j as f32) * tile_height,
                tile_width,
                tile_height,
                tile.get_color(),
            )
        })
    }

    pub fn update(&mut self, dt: f32, ants: &Vec<Ant>) {
        let (tile_width, tile_height) = self.get_tile_sizes();

        ants.iter().for_each(|ant| {
            if ant.pos().x <= 0.0
                || ant.pos().x >= screen_width()
                || ant.pos().y <= 0.0
                || ant.pos().y >= screen_height()
            {
                return;
            }
            let i = (ant.pos().x / tile_width) as usize;
            let j = (ant.pos().y / tile_height) as usize;
            let tile = &mut self.tiles[i * self.width + j];
            match ant.has_food() {
                true => {
                    tile.from_food +=
                        PheromoneTile::FROM_FOOD_CONSTANT * dt * ant.giving_off_amount();
                }
                false => {
                    tile.from_home +=
                        PheromoneTile::FROM_HOME_CONSTANT * dt * ant.giving_off_amount();
                }
            }
        });

        self.tiles.iter_mut().for_each(|tile| {
            tile.from_home = tile.from_home.clamp(0.0, 1.0);
            tile.from_food = tile.from_food.clamp(0.0, 1.0);
            tile.from_home *= 0.999;
            tile.from_food *= 0.999;
        })
    }

    pub fn get_at_point(&self, pos: Vec2) -> Option<&PheromoneTile> {
        if pos.x <= 0.0 || pos.x >= screen_width() || pos.y <= 0.0 || pos.y >= screen_height() {
            return None;
        }
        let (tile_width, tile_height) = self.get_tile_sizes();

        let i = (pos.x / tile_width) as usize;
        let j = (pos.y / tile_height) as usize;
        Some(&self.tiles[i * self.width + j])
    }
}

impl PheromoneTile {
    pub const FROM_FOOD_CONSTANT: f32 = 1.0;
    pub const FROM_HOME_CONSTANT: f32 = 1.0;

    fn get_color(&self) -> Color {
        Color::new(self.from_home, self.from_food, 0.5, 1.0)
    }
}
