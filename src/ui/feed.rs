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
                    let btn =
                        egui::Button::new(egui::RichText::new(*c).size(13.5).color(if is_sel {
                            ACCENT
                        } else {
                            GRAY
                        }))
                        .fill(if is_sel {
                            CHIP_ACTIVE_BG
                        } else {
                            egui::Color32::TRANSPARENT
                        })
                        .rounding(egui::Rounding::same(16.0));
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

        // ── Превью 16:9 ─────────────────────────────
        let size = egui::vec2(width, width * 9.0 / 16.0);

        let resp = if let Some(t) = tex {
            let img = ui.add(
                egui::Image::new(egui::load::SizedTexture::new(t.id(), size))
                    .rounding(egui::Rounding::same(8.0)),
            );
            // Image по умолчанию не кликабельна — вешаем кликабельную область поверх
            ui.interact(img.rect, img.id.with("click"), egui::Sense::click())
        } else {
            let (r, resp) = ui.allocate_exact_size(size, egui::Sense::click());
            let pulse = 0.5 + 0.5 * (ui.input(|i| i.time) * 3.0).sin();
            ui.painter().rect_filled(
                r,
                egui::Rounding::same(8.0),
                egui::Color32::from_gray((232.0 - 12.0 * pulse) as u8),
            );
            ui.ctx().request_repaint();
            resp
        };

        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if resp.clicked() {
            action = Some(CardAction::Play);
        }

        // Бейдж длительности
        if let Some(d) = video.duration {
            if d > 0 {
                let text = models::format_duration(d);
                let w = text.len() as f32 * 6.5 + 10.0;
                let badge = egui::Rect::from_min_size(
                    resp.rect.right_bottom() - egui::vec2(w + 6.0, 24.0),
                    egui::vec2(w, 18.0),
                );
                ui.painter().rect_filled(
                    badge,
                    egui::Rounding::same(4.0),
                    egui::Color32::from_black_alpha(200),
                );
                ui.painter().text(
                    badge.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
            }
        }

        // ── Автор + мета ────────────────────────────
        ui.add_space(6.0);
        ui.horizontal_top(|ui| {
            let (av, _) = ui.allocate_exact_size(egui::vec2(28.0, 28.0), egui::Sense::hover());
            ui.painter()
                .circle_filled(av.center(), 14.0, avatar_color(video.owner_id));

            ui.vertical(|ui| {
                ui.set_max_width(width - 40.0);
                ui.label(
                    egui::RichText::new(video.title.clone())
                        .strong()
                        .size(13.0)
                        .color(TEXT),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} просмотров · {}",
                        models::format_views(video.views.unwrap_or(0)),
                        models::time_ago_ru(video.date.unwrap_or(0))
                    ))
                    .size(12.0)
                    .color(GRAY),
                );
            });
        });

        if ui.small_button("⬇ Скачать").clicked() {
            action = Some(CardAction::Download);
        }
    });

    action
}

fn avatar_color(id: i64) -> egui::Color32 {
    const PAL: &[egui::Color32] = &[
        egui::Color32::from_rgb(255, 107, 107),
        egui::Color32::from_rgb(76, 175, 80),
        egui::Color32::from_rgb(33, 150, 243),
        egui::Color32::from_rgb(156, 39, 176),
        egui::Color32::from_rgb(255, 152, 0),
        egui::Color32::from_rgb(0, 150, 136),
    ];
    PAL[(id.unsigned_abs() as usize) % PAL.len()]
}
