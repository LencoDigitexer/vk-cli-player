mod api;
mod app;
mod models;
mod services;
mod ui;

use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "VK Видео",
        options,
        Box::new(|cc| {
            ui::theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::VkVideoApp::new()))
        }),
    )?;
    Ok(())
}
