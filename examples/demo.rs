use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

fn draw_primitives() {
    draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
    draw_rectangle(-0.1, 0.1, 0.2, 0.2, GREEN);
    draw_circle(0., 0., 0.1, YELLOW);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut show_egui_demo_windows = false;
    let mut egui_demo_windows = egui_demo_lib::DemoWindows::default();
    let mut draw_primitives_after_egui = false;

    let mut pixels_per_point: Option<f32> = None;

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            if pixels_per_point.is_none() {
                pixels_per_point = Some(egui_ctx.pixels_per_point());
            }

            if show_egui_demo_windows {
                egui_demo_windows.ui(egui_ctx);
            }

            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.checkbox(&mut show_egui_demo_windows, "Show egui demo windows");
                ui.checkbox(
                    &mut draw_primitives_after_egui,
                    "Draw macroquad primitives after egui",
                );

                let response = ui.add(
                    egui::Slider::new(pixels_per_point.as_mut().unwrap(), 0.75..=3.0)
                        .logarithmic(true),
                );

                // Don't change scale while dragging the slider
                if response.drag_stopped() {
                    egui_ctx.set_pixels_per_point(pixels_per_point.unwrap());
                }
            });
        });

        set_camera(&Camera2D {
            zoom: vec2(1., -screen_width() / screen_height()),
            ..Default::default()
        });

        if !draw_primitives_after_egui {
            draw_primitives();
        }
        egui_macroquad::draw();
        if draw_primitives_after_egui {
            draw_primitives();
        }

        next_frame().await
    }
}
