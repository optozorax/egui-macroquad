# [egui](https://github.com/emilk/egui) bindings for [macroquad](https://github.com/not-fl3/macroquad)

[![Latest version](https://img.shields.io/crates/v/egui-macroquad.svg)](https://crates.io/crates/egui-macroquad)
[![Documentation](https://docs.rs/egui-macroquad/badge.svg)](https://docs.rs/egui-macroquad)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

This is the easiest way to use egui. Just two functions!

[Web demo.](https://optozorax.github.io/egui-macroquad/)

# Usage

You need to call `ui` when you need to get information from ui. Then, only after that function you must call `draw` function when you need to draw egui contents. All this functions should be called each frame and once per frame.

Here is the small example on how to use this library: 
```rust 
use macroquad::prelude::*;

#[macroquad::main("egui with macroquad")]
async fn main() {
    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad")
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });

        // Draw things before egui

        egui_macroquad::draw();
        
        // Draw things after egui

        next_frame().await;
    }
}
```

# Building

Building for native and for web works just as in `macroquad`. You can read about it [here](https://github.com/not-fl3/miniquad/#building-examples). Or you could look at building example at [egui-miniquad](https://github.com/not-fl3/egui-miniquad).

But for wasm you will need to include two more `.js` files, which is plugins for quads, instruction is written [here](https://github.com/optozorax/quad-url).