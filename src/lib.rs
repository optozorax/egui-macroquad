//! # egui bindings for macroquad
//!
//! This is the easiest way to use egui. Just two functions!
//!
//! [Web demo.](https://optozorax.github.io/egui-macroquad/)
//!
//! # Usage
//!
//! You need to call [`ui`] when you need to get information from ui. Then, only after that function you must call [`draw`] function when you need to draw egui contents. All this functions should be called each frame and once per frame.
//!
//! Here is the small example on how to use this library:
//! ```rust
//! use macroquad::prelude::*;
//!
//! #[macroquad::main("egui with macroquad")]
//! async fn main() {
//!     loop {
//!         clear_background(WHITE);
//!
//!         // Process keys, mouse etc.
//!
//!         egui_macroquad::ui(|egui_ctx| {
//!             egui::Window::new("egui ❤ macroquad")
//!                 .show(egui_ctx, |ui| {
//!                     ui.label("Test");
//!                 });
//!         });
//!
//!         // Draw things before egui
//!
//!         egui_macroquad::draw();
//!
//!         // Draw things after egui
//!
//!         next_frame().await;
//!     }
//! }
//! ```
//!
//! # Building
//!
//! Building for native and for web works just as in `macroquad`. You can read about it [here](https://github.com/not-fl3/miniquad/#building-examples). Or you could look at building example at [egui-miniquad](https://github.com/not-fl3/egui-miniquad).
//!
//! But for wasm you will need to include two more `.js` files, which is plugins for quads, instruction is written [here](https://github.com/optozorax/quad-url).

use egui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;

pub use egui;
pub use macroquad;

struct Egui {
    egui_mq: EguiMq,
    input_subscriber_id: usize,
}

// Global variable and global functions because it's more like macroquad way
static mut EGUI: Option<Egui> = None;

fn get_egui() -> &'static mut Egui {
    unsafe {
        if let Some(egui) = EGUI.as_mut() {
            egui
        } else {
            EGUI = Some(Egui::new());
            EGUI.as_mut().unwrap()
        }
    }
}

impl Egui {
    fn new() -> Self {
        Self {
            egui_mq: EguiMq::new(unsafe { get_internal_gl() }.quad_context),
            input_subscriber_id: macroquad::input::utils::register_input_subscriber(),
        }
    }

    fn ui<F>(&mut self, f: F)
    where
        F: FnMut(&mut dyn mq::RenderingBackend, &egui::Context),
    {
        let gl = unsafe { get_internal_gl() };
        macroquad::input::utils::repeat_all_miniquad_input(self, self.input_subscriber_id);

        self.egui_mq.run(gl.quad_context, f);
    }

    fn draw(&mut self) {
        let mut gl = unsafe { get_internal_gl() };
        // Ensure that macroquad's shapes are not goint to be lost, and draw them now
        gl.flush();
        self.egui_mq.draw(gl.quad_context);
    }
}

/// Calculates egui ui. Must be called once per frame.
pub fn ui<F: FnMut(&egui::Context)>(mut f: F) {
    get_egui().ui(|_, ctx| f(ctx))
}

/// Configure egui without beginning or ending a frame.
pub fn cfg<F: FnOnce(&egui::Context)>(f: F) {
    f(get_egui().egui_mq.egui_ctx());
}

/// Draw egui ui. Must be called after `ui` and once per frame.
pub fn draw() {
    get_egui().draw()
}

// Intended to be used only if you recreate the window, making the old EGUI instance invalid.
#[doc(hidden)]
pub fn reset_egui() {
    unsafe {
        EGUI = None;
    }
}

impl mq::EventHandler for Egui {
    fn update(&mut self) {}

    fn draw(&mut self) {}

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_down_event(mb, x, y);
    }

    fn mouse_button_up_event(&mut self, mb: mq::MouseButton, x: f32, y: f32) {
        self.egui_mq.mouse_button_up_event(mb, x, y);
    }

    fn char_event(&mut self, character: char, _keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods, _repeat: bool) {
        self.egui_mq.key_down_event(keycode, keymods);
    }

    fn key_up_event(&mut self, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }
}
