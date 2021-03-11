use emigui_miniquad::EguiMq;
use macroquad::prelude::*;
use miniquad as mq;

pub struct Egui(EguiMq);

impl Egui {
    pub fn new() -> Self {
        Self(EguiMq::new(unsafe { get_internal_gl() }.quad_context))
    }

    pub fn ui<F: FnOnce(&egui::CtxRef)>(&mut self, f: F) {
        // Ensure that macroquad's shapes are not goint to be lost
        let mut gl = unsafe { get_internal_gl() };
        gl.flush();

        repeat_all_miniquad_input(self);

        self.0.begin_frame(gl.quad_context);
        f(self.0.egui_ctx());
        self.0.end_frame(gl.quad_context);
    }
}

impl mq::EventHandler for Egui {
    fn update(&mut self, _ctx: &mut mq::Context) {}

    fn draw(&mut self, _ctx: &mut mq::Context) {}

    fn mouse_motion_event(&mut self, ctx: &mut mq::Context, x: f32, y: f32) {
        self.0.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut mq::Context, dx: f32, dy: f32) {
        self.0.mouse_wheel_event(ctx, dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.0.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut mq::Context,
        mb: mq::MouseButton,
        x: f32,
        y: f32,
    ) {
        self.0.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut mq::Context,
        character: char,
        _keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.0.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut mq::Context,
        keycode: mq::KeyCode,
        keymods: mq::KeyMods,
        _repeat: bool,
    ) {
        self.0.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut mq::Context, keycode: mq::KeyCode, keymods: mq::KeyMods) {
        self.0.key_up_event(keycode, keymods);
    }
}
