// feed.rs — чипсы, карточка, скелетон
use crate::models::{self, VideoItem};
use crate::ui::theme::*;
use eframe::egui;

pub const CATEGORIES: &[&str] = &[
    "Все",
    "Интервью и шоу",
    "Политика",
    "Музыка",
    "Путешествия",
    "Авто",
    "Технологии",
    "Еда",
    "Образование",
    "Мода и красота",
    "Здоровье",
    "Интерактив",
    "Культура",
];

pub enum CardAction {
    Play,
    Download,
}

pub fn chips(ui: &mut egui::Ui, selected: &str) -> Option<String> {
    let mut picked = None;
    egui::ScrollArea::horizontal()
        .id_source("chips")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for c in CATEGORIES {
                    let is_sel = *c == selected;
                    
                    let bg_color = if is_sel { ACCENT } else { CHIP_BG };
                    let text_color = if is_sel { TEXT_PRIMARY } else { TEXT_SECONDARY };
                    
                    let btn =
                        egui::Button::new(egui::RichText::new(*c).size(13.5).color(text_color))
                            .fill(bg_color)
                            .rounding(egui::Rounding::same(20.0))
                            .stroke(if is_sel {
                                egui::Stroke::NONE
                            } else {
                                egui::Stroke::new(1.0, DIVIDER)
                            });
                    if ui.add(btn).clicked() {
                        picked = Some(c.to_string());
                    }
                }
            });
        });
    picked
}

pub fn card(
    ui: &mut egui::Ui,
    video: &VideoItem,
    tex: Option<&egui::TextureHandle>,
    width: f32,
) -> Option<CardAction> {
    let mut action = None;

    ui.vertical(|ui| {
        ui.set_max_width(width);

        // ── Превью 16:9 с тенью ─────────────────────────────
        let size = egui::vec2(width, width * 9.0 / 16.0);

        let (rect, resp) = if let Some(t) = tex {
            let img = ui.add(
                egui::Image::new(egui::load::SizedTexture::new(t.id(), size))
                    .rounding(egui::Rounding::same(12.0)),
            );
            // Image по умолчанию не кликабельна — вешаем кликабельную область поверх
            (img.rect, ui.interact(img.rect, img.id.with("click"), egui::Sense::click()))
        } else {
            let (r, resp) = ui.allocate_exact_size(size, egui::Sense::click());
            let pulse = 0.5 + 0.5 * (ui.input(|i| i.time) * 2.0).sin();
            
            // Градиентный фон для скелетона
            let grad_rect = r;
            ui.painter().rect_filled(
                grad_rect,
                egui::Rounding::same(12.0),
                egui::Color32::from_rgb((40.0 - 5.0 * pulse) as u8, (40.0 - 5.0 * pulse) as u8, (50.0 - 5.0 * pulse) as u8),
            );
            
            ui.ctx().request_repaint();
            (r, resp)
        };

        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            
            // Эффект затемнения при наведении
            ui.painter().rect_filled(
                rect,
                egui::Rounding::same(12.0),
                egui::Color32::from_black_alpha(60),
            );
            
            // Кнопка Play по центру при наведении
            let play_size = 50.0;
            let play_rect = egui::Rect::from_center_size(
                rect.center(),
                egui::vec2(play_size, play_size),
            );
            ui.painter().circle_filled(
                play_rect.center(),
                play_size / 2.0,
                PLAY_BUTTON_BG,
            );
            ui.painter().text(
                play_rect.center() + egui::vec2(3.0, 0.0),
                egui::Align2::CENTER_CENTER,
                "▶",
                egui::FontId::proportional(24.0).weight(egui::FontWeight::BOLD),
                egui::Color32::WHITE,
            );
        }
        
        if resp.clicked() {
            action = Some(CardAction::Play);
        }

        // Бейдж длительности
        if let Some(d) = video.duration {
            if d > 0 {
                let text = models::format_duration(d);
                let w = text.len() as f32 * 7.0 + 12.0;
                let badge = egui::Rect::from_min_size(
                    rect.right_bottom() - egui::vec2(w + 8.0, 28.0),
                    egui::vec2(w, 20.0),
                );
                ui.painter().rect_filled(
                    badge,
                    egui::Rounding::same(5.0),
                    egui::Color32::from_black_alpha(220),
                );
                ui.painter().text(
                    badge.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(12.0).weight(egui::FontWeight::MEDIUM),
                    egui::Color32::WHITE,
                );
            }
        }

        // ── Автор + мета ────────────────────────────
        ui.add_space(10.0);
        ui.horizontal_top(|ui| {
            let (av, _) = ui.allocate_exact_size(egui::vec2(36.0, 36.0), egui::Sense::hover());
            ui.painter()
                .circle_filled(av.center(), 18.0, avatar_color(video.owner_id));

            ui.vertical(|ui| {
                ui.set_max_width(width - 50.0);
                ui.label(
                    egui::RichText::new(video.title.clone())
                        .strong()
                        .size(14.0)
                        .color(TEXT_PRIMARY),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} просмотров · {}",
                        models::format_views(video.views.unwrap_or(0)),
                        models::time_ago_ru(video.date.unwrap_or(0))
                    ))
                    .size(12.0)
                    .color(TEXT_MUTED),
                );
            });
        });

        // Кнопка скачать стилизованная
        ui.add_space(4.0);
        let download_btn = egui::Button::new(
            egui::RichText::new("⬇ Скачать").size(12.0).color(TEXT_SECONDARY),
        )
        .fill(BG_HOVER)
        .rounding(egui::Rounding::same(8.0))
        .stroke(egui::Stroke::new(1.0, DIVIDER));
        
        if ui.add(download_btn).clicked() {
            action = Some(CardAction::Download);
        }
    });

    action
}

fn avatar_color(id: i64) -> egui::Color32 {
    const PAL: &[egui::Color32] = &[
        egui::Color32::from_rgb(255, 82, 82),
        egui::Color32::from_rgb(76, 175, 80),
        egui::Color32::from_rgb(33, 150, 243),
        egui::Color32::from_rgb(156, 39, 176),
        egui::Color32::from_rgb(255, 152, 0),
        egui::Color32::from_rgb(0, 188, 212),
    ];
    PAL[(id.unsigned_abs() as usize) % PAL.len()]
}
