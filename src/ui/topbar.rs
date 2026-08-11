// topbar.rs
use crate::ui::theme::*;
use eframe::egui;

/// Возвращает true, если пользователь отправил поиск (Enter)
pub fn show(ui: &mut egui::Ui, query: &mut String) -> bool {
    let mut submitted = false;

    ui.horizontal_centered(|ui| {
        // Лого
        let (logo, _) = ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());
        ui.painter()
            .rect_filled(logo, egui::Rounding::same(8.0), ACCENT);
        ui.painter().text(
            logo.center(),
            egui::Align2::CENTER_CENTER,
            "▶",
            egui::FontId::proportional(11.0),
            egui::Color32::WHITE,
        );
        ui.label(
            egui::RichText::new("VK Видео")
                .strong()
                .size(16.0)
                .color(TEXT),
        );
        ui.add_space(24.0);

        // Поиск по центру (скругление делаем рамкой Frame — TextEdit в 0.28 этого не умеет)
        let w = (ui.available_width() * 0.6).min(560.0);
        let resp = egui::Frame::default()
            .fill(PANEL)
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(egui::Margin::symmetric(10.0, 5.0))
            .show(ui, |ui| {
                ui.add_sized(
                    [w, 26.0],
                    egui::TextEdit::singleline(query)
                        .hint_text("Поиск видео")
                        .frame(false),
                )
            })
            .inner;
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            submitted = true;
        }

        // Правый блок
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (av, _) = ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());
            ui.painter().circle_filled(av.center(), 14.0, GRAY);
            ui.add(
                egui::Button::new(egui::RichText::new("🔔").size(15.0))
                    .fill(egui::Color32::TRANSPARENT),
            );
            ui.add(
                egui::Button::new(egui::RichText::new("＋").size(15.0))
                    .fill(egui::Color32::TRANSPARENT),
            );
        });
    });

    submitted
}
