// topbar.rs
use crate::ui::theme::*;
use eframe::egui;

/// Возвращает true, если пользователь отправил поиск (Enter)
pub fn show(ui: &mut egui::Ui, query: &mut String) -> bool {
    let mut submitted = false;

    ui.horizontal_centered(|ui| {
        // Логотип с градиентом
        let (logo, _) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::hover());
        
        // Рисуем логотип с закруглением и градиентом
        ui.painter().rect_filled(
            logo, 
            egui::Rounding::same(10.0), 
            ACCENT_GRADIENT_START
        );
        
        // Добавляем эффект блика
        let highlight = egui::Rect::from_min_size(
            logo.min,
            egui::vec2(logo.width(), logo.height() * 0.4),
        );
        ui.painter().rect_filled(
            highlight,
            egui::Rounding {
                nw: 10.0,
                ne: 10.0,
                sw: 0.0,
                se: 0.0,
            },
            ACCENT_GRADIENT_END,
        );
        
        ui.painter().text(
            logo.center(),
            egui::Align2::CENTER_CENTER,
            "▶",
            egui::FontId::proportional(14.0).weight(egui::FontWeight::BOLD),
            egui::Color32::WHITE,
        );
        
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new("VK Видео")
                .strong()
                .size(18.0)
                .color(TEXT_PRIMARY),
        );
        ui.add_space(32.0);

        // Поиск по центру
        let w = (ui.available_width() * 0.6).min(560.0);
        let resp = egui::Frame::default()
            .fill(BG_SECONDARY)
            .rounding(egui::Rounding::same(12.0))
            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
            .stroke(egui::Stroke::new(1.0, DIVIDER))
            .show(ui, |ui| {
                ui.add_sized(
                    [w, 32.0],
                    egui::TextEdit::singleline(query)
                        .hint_text("Поиск видео")
                        .font(egui::TextStyle::Body)
                        .frame(false),
                )
            })
            .inner;
        if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            submitted = true;
        }

        // Правый блок с кнопками
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Аватар
            let (av, av_resp) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::click());
            ui.painter().circle_filled(av.center(), 18.0, ACCENT);
            
            if av_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            
            ui.add_space(8.0);
            
            // Кнопка уведомлений
            let bell_btn = egui::Button::new(egui::RichText::new("🔔").size(18.0))
                .fill(egui::Color32::TRANSPARENT)
                .rounding(egui::Rounding::same(8.0));
            let bell_resp = ui.add(bell_btn);
            if bell_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
            
            ui.add_space(4.0);
            
            // Кнопка добавления
            let add_btn = egui::Button::new(egui::RichText::new("＋").size(18.0))
                .fill(egui::Color32::TRANSPARENT)
                .rounding(egui::Rounding::same(8.0));
            let add_resp = ui.add(add_btn);
            if add_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        });
    });

    submitted
}
