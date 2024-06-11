use canvas::{circle, clear, fill, line, rect, stroke, Canvas};
use once_cell::sync::Lazy;
use std::f64::consts::PI;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod canvas;

const PIXEL_SIZE: f64 = 32.0;
const MAP_NUM_ROWS: usize = 11;
const MAP_NUM_COLS: usize = 15;
const MAP_WIDTH: f64 = MAP_NUM_COLS as f64 * PIXEL_SIZE;
const MAP_HEIGHT: f64 = MAP_NUM_ROWS as f64 * PIXEL_SIZE;

static GRID: Lazy<Grid> = Lazy::new(|| Grid::new());
static mut PLAYER: Lazy<Player> = Lazy::new(|| Player::new());

struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
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

    // 是否是墙
    fn has_wall(&self, x: f64, y: f64) -> bool {
        if x < 0.0 || x > MAP_WIDTH || y < 0.0 || y > MAP_HEIGHT {
            return true;
        }

        let wall_y = (y / PIXEL_SIZE).floor() as usize;
        let wall_x = (x / PIXEL_SIZE).floor() as usize;

        self.grid[wall_y][wall_x] == 1
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
    pub moving_speed: f64,
    pub turning_speed: f64,
    // 方向 -1 后退 1 前进 0 不动
    pub direction: i8,
    // 旋转方向
    // -1 左边 1 右边  0 不动
    pub turning_direction: i8,
}

impl Player {
    fn new() -> Self {
        Self {
            x: MAP_HEIGHT / 2.0,
            y: MAP_HEIGHT / 2.0,
            ray_radius: 90.0 * (PI / 180.0),
            moving_speed: 1.0,
            turning_speed: 3.0 * (PI / 180.0),
            direction: 0,
            turning_direction: 0,
        }
    }

    fn update(&mut self) {
        if self.direction != 0 {
            let x = self.x + self.ray_radius.cos() * self.moving_speed * self.direction as f64;
            let y = self.y + self.ray_radius.sin() * self.moving_speed * self.direction as f64;

            if !GRID.has_wall(x, y) {
                self.x = x;
                self.y = y;
            }
        }

        if self.turning_direction != 0 {
            self.ray_radius += self.turning_speed * self.turning_direction as f64;
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
fn setup() {
    Canvas::new(
        Rc::new(RefCell::new(draw)),
        Rc::new(RefCell::new(key_down)),
        Rc::new(RefCell::new(key_up)),
    )
    .run();
}

fn update() {
    unsafe {
        PLAYER.update();
    }
}

fn draw() {
    clear();

    GRID.render();
    unsafe {
        PLAYER.render();
    }

    update();
}

fn key_down(key: &str) {
    unsafe {
        match key {
            "W" => PLAYER.direction = 1,
            "S" => PLAYER.direction = -1,
            "A" => PLAYER.turning_direction = -1,
            "D" => PLAYER.turning_direction = 1,
            _ => (),
        }
    }
}

fn key_up(key: &str) {
    unsafe {
        match key {
            "W" => PLAYER.direction = 0,
            "S" => PLAYER.direction = 0,
            "A" => PLAYER.turning_direction = 0,
            "D" => PLAYER.turning_direction = 0,
            _ => (),
        }
    }
}
