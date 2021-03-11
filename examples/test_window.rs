use egui_macroquad::Egui;
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
    let mut egui = Egui::new();

    let mut show_egui_demo_windows = false;
    let mut egui_demo_windows = egui_demo_lib::DemoWindows::default();

    loop {
        clear_background(RED);

        // Render some primitives in camera space

        set_camera(Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);


        egui.ui(|egui_ctx| {
            if show_egui_demo_windows {
                egui_demo_windows.ui(egui_ctx);
            }

            egui::Window::new("Debug").show(egui_ctx, |ui| {
                ui.add(egui::Label::new("Egui on Macroquad").text_style(egui::TextStyle::Heading));
                ui.separator();
                ui.checkbox(&mut show_egui_demo_windows, "Show egui demo windows");
                ui.label("Woooohoooo!");
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
        });

        // Render circle after egui

        draw_circle(0., 0., 0.1, YELLOW);

        next_frame().await
    }
}
