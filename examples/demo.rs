use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut show_egui_demo_windows = false;
    let mut egui_demo_windows = egui_demo_lib::DemoWindows::default();

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            if show_egui_demo_windows {
                egui_demo_windows.ui(egui_ctx);
            }

            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.checkbox(&mut show_egui_demo_windows, "Show egui demo windows");
            });
        });

        set_camera(Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.1, 0.1, 0.2, 0.2, GREEN);
        egui_macroquad::draw();
        draw_circle(0., 0., 0.1, YELLOW);

        next_frame().await
    }
}
