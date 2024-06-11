use crate::Closure;
use crate::JsValue;
use once_cell::sync::Lazy;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::CanvasRenderingContext2d;

type DrawFn = Rc<RefCell<dyn FnMut() + 'static>>;

pub type KeyBoard = Rc<RefCell<dyn FnMut(&str) + 'static>>;

pub static mut CANVAS: Lazy<Option<web_sys::HtmlCanvasElement>> = Lazy::new(|| None);

pub fn create_offset_canvas() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .ok();

    if let Some(ref canvas) = canvas {
        canvas.set_width(600);
        canvas.set_height(800);
    }

    unsafe {
        *CANVAS = canvas;
    }
}

pub fn get_content() -> CanvasRenderingContext2d {
    unsafe {
        let context = CANVAS
            .clone()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .ok();
        context.unwrap()
    }
}

pub struct Canvas {
    pub draw: DrawFn,
    pub key_down: KeyBoard,
    pub key_up: KeyBoard,
}

impl Canvas {
    pub fn new(draw: DrawFn, key_down: KeyBoard, key_up: KeyBoard) -> Self {
        Canvas {
            // context: None,
            draw: draw,
            key_down: key_down,
            key_up: key_up,
        }
    }

    pub fn run(&self) {
        create_offset_canvas();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let draw = self.draw.clone();
        let key_down = self.key_down.clone();
        let key_up = self.key_up.clone();

        let keydown = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key().to_uppercase();
            let key = key.as_str();
            key_down.borrow_mut()(key);
        });

        let keyup = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            let key = event.key().to_uppercase();
            let key = key.as_str();
            key_up.borrow_mut()(key);
        });

        document
            .add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref())
            .unwrap();

        document
            .add_event_listener_with_callback("keyup", keyup.as_ref().unchecked_ref())
            .unwrap();

        keyup.forget();
        keydown.forget();

        *g.borrow_mut() = Some(Closure::new(move || {
            // Set the body's text content to how many times this
            draw.borrow_mut()();
            unsafe {
                context
                    .draw_image_with_html_canvas_element(CANVAS.as_ref().unwrap(), 0.0, 0.0)
                    .unwrap();
            }
            // Schedule ourself for another requestAnimationFrame callback.
            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn clear() {
    let content = get_content();
    content.clear_rect(0.0, 0.0, 600.0, 800.0);
}

pub fn fill(color: &str) {
    let ctx = get_content();
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill();
}

pub fn stroke(color: &str) {
    let ctx = get_content();
    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_line_width(1.0);
    ctx.stroke();
}

pub fn rect(x: f64, y: f64, w: f64, h: f64) {
    let ctx = get_content();
    ctx.begin_path();
    ctx.rect(x, y, w, h);
}

pub fn circle(x: f64, y: f64, radius: f64) {
    let ctx = get_content();
    ctx.begin_path();
    ctx.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)
        .unwrap();
}

pub fn line(x1: f64, y1: f64, x2: f64, y2: f64) {
    let ctx = get_content();
    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke();
}
