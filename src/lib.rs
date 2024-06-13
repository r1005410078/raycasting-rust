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
// 墙的宽度
const WALL_WIDTH: f64 = 1.0;
// 视角
const VIEW_ANGLE: f64 = 60.0 * (PI / 180.0);

static GRID: Lazy<Grid> = Lazy::new(|| Grid::new());
static mut PLAYER: Lazy<Player> = Lazy::new(|| Player::new());
static mut RAYS: Lazy<Vec<Ray>> = Lazy::new(|| Vec::new());

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
            ray_radius: 354.0 * (PI / 180.0),
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
        stroke("blue");
        circle(self.x, self.y, 3.0);
        fill("blue");

        // 偏移角度、
        unsafe {
            RAYS.clear();
            let mut angle = PLAYER.ray_radius - VIEW_ANGLE / 2.0;
            let angle_step = VIEW_ANGLE / MAP_WIDTH as f64;

            for _column_id in 0..MAP_WIDTH as usize {
                let mut ray = Ray::new(angle);
                ray.cast();

                RAYS.push(ray);

                angle += angle_step;
            }
        }
    }
}

struct Ray {
    // 角度
    ray_angle: f64,
    distance: f64,
    is_ray_facing_down: bool,
    is_ray_facing_up: bool,
    is_ray_facing_left: bool,
    is_ray_facing_right: bool,
    wall_x: f64,
    wall_y: f64,
}

impl Ray {
    fn new(angle: f64) -> Self {
        let ray_angle = normalize_angle(angle);
        let is_ray_facing_down = ray_angle >= 0.0 && ray_angle <= PI;
        let is_ray_facing_up = !is_ray_facing_down;
        let is_ray_facing_right = ray_angle < 0.5 * PI || ray_angle > 1.5 * PI;
        let is_ray_facing_left = !is_ray_facing_right;

        // console::log_1(&JsValue::from(
        //     format!(
        //         "is_ray_facing_down: {}, is_ray_facing_up: {}, is_ray_facing_left: {}, is_ray_facing_right: {}",
        //         is_ray_facing_down,
        //         is_ray_facing_up,
        //         is_ray_facing_left,
        //         is_ray_facing_right
        //     ).as_str()
        // ));

        Self {
            ray_angle,
            distance: 0.0,
            is_ray_facing_down,
            is_ray_facing_up,
            is_ray_facing_left,
            is_ray_facing_right,
            wall_x: 0.0,
            wall_y: 0.0,
        }
    }

    unsafe fn cast(&mut self) {
        let mut xinstercept = 0.0;
        let mut yinstercept = 0.0;
        let mut xsteep = 0.0;
        let mut ysteep = 0.0;

        let mut found_horz_wallhit = false;
        let mut horz_wall_hit_x = 0.0;
        let mut horz_wall_hit_y = 0.0;

        yinstercept = (PLAYER.y / PIXEL_SIZE).floor() * PIXEL_SIZE;
        if self.is_ray_facing_down {
            yinstercept += PIXEL_SIZE;
        }

        xinstercept = PLAYER.x + (yinstercept - PLAYER.y) / self.ray_angle.tan();

        ysteep = PIXEL_SIZE;
        if self.is_ray_facing_up {
            ysteep *= -1.0;
        }

        xsteep = PIXEL_SIZE / self.ray_angle.tan();

        if self.is_ray_facing_up {
            xsteep *= -1.0;
        }

        // if xsteep > 0.0 && self.is_ray_facing_left {
        //     xinstercept *= -1.0;
        // }

        // if xsteep < 0.0 && self.is_ray_facing_right {
        //     xinstercept *= -1.0;
        // }

        let mut next_horz_touch_x = xinstercept;
        let mut next_horz_touch_y = yinstercept;

        while horz_wall_hit_x >= 0.0
            || horz_wall_hit_x <= MAP_WIDTH
            || horz_wall_hit_y >= 0.0
            || horz_wall_hit_y <= MAP_HEIGHT
        {
            if self.is_ray_facing_up {
                next_horz_touch_y -= 1.0;
            }

            if GRID.has_wall(next_horz_touch_x, next_horz_touch_y) {
                found_horz_wallhit = true;

                horz_wall_hit_x = next_horz_touch_x;
                horz_wall_hit_y = next_horz_touch_y;
                break;
            } else {
                next_horz_touch_x += xsteep;
                next_horz_touch_y += ysteep;
            }
        }

        let mut found_vert_wallhit = false;
        let mut vert_wall_hit_x = 0.0;
        let mut vert_wall_hit_y = 0.0;

        xinstercept = (PLAYER.x / PIXEL_SIZE).floor() * PIXEL_SIZE;

        if self.is_ray_facing_right {
            xinstercept += PIXEL_SIZE;
        }

        yinstercept = PLAYER.y + (xinstercept - PLAYER.x) * self.ray_angle.tan();

        xsteep = PIXEL_SIZE;
        if self.is_ray_facing_left {
            xsteep *= -1.0;
        }

        ysteep = PIXEL_SIZE * self.ray_angle.tan();

        if self.is_ray_facing_down && ysteep < 0.0 {
            ysteep *= -1.0;
        }

        if self.is_ray_facing_up && ysteep > 0.0 {
            ysteep *= -1.0;
        }

        let mut next_vert_touch_x = xinstercept;
        let mut next_vert_touch_y = yinstercept;

        while vert_wall_hit_x >= 0.0
            || vert_wall_hit_x <= MAP_WIDTH
            || vert_wall_hit_y >= 0.0
            || vert_wall_hit_y <= MAP_HEIGHT
        {
            if self.is_ray_facing_left {
                next_vert_touch_x -= 1.0;
            }

            if GRID.has_wall(next_vert_touch_x, next_vert_touch_y) {
                found_vert_wallhit = true;

                vert_wall_hit_x = next_vert_touch_x;
                vert_wall_hit_y = next_vert_touch_y;
                break;
            } else {
                next_vert_touch_x += xsteep;
                next_vert_touch_y += ysteep;
            }
        }

        let horz_hit_distance = if found_horz_wallhit {
            distance_between_points(PLAYER.x, PLAYER.y, horz_wall_hit_x, horz_wall_hit_y)
        } else {
            std::f64::MAX
        };

        let vert_hit_distance = if found_vert_wallhit {
            distance_between_points(PLAYER.x, PLAYER.y, vert_wall_hit_x, vert_wall_hit_y)
        } else {
            std::f64::MAX
        };

        self.wall_x = if horz_hit_distance < vert_hit_distance {
            horz_wall_hit_x
        } else {
            vert_wall_hit_x
        };

        self.wall_y = if horz_hit_distance < vert_hit_distance {
            horz_wall_hit_y
        } else {
            vert_wall_hit_y
        };

        // console::log_1(&JsValue::from(format!(
        //     "tan: {} ray_angle: {}",
        //     horz_wall_hit_x, horz_wall_hit_y
        // )));
    }

    fn render(&self) {
        unsafe {
            line(
                PLAYER.x,
                PLAYER.y,
                self.wall_x,
                self.wall_y,
                // PLAYER.x + self.angle.cos() * 30.0,
                // PLAYER.y + self.angle.sin() * 30.0,
            );
            fill("red");
            stroke("red");
        }
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

    unsafe {
        for ray in RAYS.iter() {
            ray.render();
        }
    }
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

fn normalize_angle(angle: f64) -> f64 {
    let mut angle = angle % (2.0 * PI);

    if angle < 0.0 {
        angle = 2.0 * PI + angle
    }

    angle
}

fn distance_between_points(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    return ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt();
}
