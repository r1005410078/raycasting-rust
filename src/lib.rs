use canvas::{circle, clear, fill, line, rect, stroke, Canvas};
use once_cell::sync::Lazy;
use std::f64::consts::PI;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
mod canvas;

const PIXEL_SIZE: f64 = 32.0;
const MAP_NUM_ROWS: usize = 11;
const MAP_NUM_COLS: usize = 15;
const MAP_WIDTH: f64 = MAP_NUM_COLS as f64 * PIXEL_SIZE;
const MAP_HEIGHT: f64 = MAP_NUM_ROWS as f64 * PIXEL_SIZE;

static GRID: Lazy<Grid> = Lazy::new(|| Grid::new());
static PLAYER: Lazy<Player> = Lazy::new(|| Player::new());

struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
                vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
        }
    }

    fn render(&self) {
        for row in 0..MAP_NUM_ROWS {
            for col in 0..MAP_NUM_COLS {
                rect(
                    col as f64 * PIXEL_SIZE,
                    row as f64 * PIXEL_SIZE,
                    PIXEL_SIZE,
                    PIXEL_SIZE,
                );

                if self.grid[row][col] == 1 {
                    fill("rgba(0, 0, 0, 1)");
                } else {
                    fill("rgba(255, 255, 255, 1)");
                }
                stroke("black");
            }
        }
    }
}

struct Player {
    pub x: f64,
    pub y: f64,
    pub ray_radius: f64,
}

impl Player {
    fn new() -> Self {
        Self {
            x: MAP_HEIGHT / 2.0,
            y: MAP_HEIGHT / 2.0,
            ray_radius: 90.0 * (PI / 180.0),
        }
    }

    fn render(&self) {
        line(
            self.x,
            self.y,
            self.x + self.ray_radius.cos() * 20.0,
            self.y + self.ray_radius.sin() * 20.0,
        );
        stroke("red");

        circle(self.x, self.y, 3.0);
        fill("red");
    }
}

#[wasm_bindgen(start)]
fn start() {
    Canvas::new(Rc::new(RefCell::new(draw))).run();
}

fn draw() {
    clear();

    GRID.render();
    PLAYER.render();
}
