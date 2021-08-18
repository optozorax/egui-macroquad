use egui::{FontDefinitions, FontFamily, TextStyle, Window as EguiWindow};
use egui_demo_lib::DemoWindows;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "egui with macroquad".to_owned(),
        high_dpi: false,
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
    let mut egui_demo_windows = DemoWindows::default();
    let mut draw_primitives_after_egui = false;

    // setup (cfg)
    egui_macroquad::cfg(|ctx| {
        let mut fonts = FontDefinitions::default();
        let monospace = (FontFamily::Monospace, 13.0);
        fonts.family_and_size.insert(TextStyle::Button, monospace);
        fonts.family_and_size.insert(TextStyle::Body, monospace);
        fonts
            .family_and_size
            .insert(TextStyle::Monospace, monospace);
        fonts.family_and_size.insert(TextStyle::Small, monospace);
        ctx.set_fonts(fonts)
    });

    loop {
        clear_background(WHITE);

        egui_macroquad::ui(|egui_ctx| {
            if show_egui_demo_windows {
                egui_demo_windows.ui(egui_ctx);
            }
            EguiWindow::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.checkbox(&mut show_egui_demo_windows, "Show egui demo windows");
                ui.checkbox(
                    &mut draw_primitives_after_egui,
                    "Draw macroquad primitives after egui",
                );
            });
        });
        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });

        if !draw_primitives_after_egui {
            draw_primitives();
        }
        egui_macroquad::draw();
        if draw_primitives_after_egui {
            draw_primitives();
        }
        next_frame().await;
        // // reduce CPU usage on native (non-web) if need to, put the main thread to sleep for a while after next_frame().
        // std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
