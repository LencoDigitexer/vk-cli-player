// sidebar.rs
use crate::ui::theme::*;
use eframe::egui;

pub enum Nav {
    Home,
    Query(String),
}

const MAIN: &[(&str, &str)] = &[
    ("📡", "Стримы"),
    ("🏠", "Главная"),
    ("⚡", "Тренды"),
    ("🎞", "Клипы"),
];

const LIBRARY: &[(&str, &str)] = &[
    ("🕒", "История просмотра"),
    ("🕓", "Смотреть позже"),
    ("❤", "Мне понравилось"),
    ("📋", "Мои плейлисты"),
    ("🎥", "Кабинет автора"),
];

const CATS: &[(&str, &str)] = &[
    ("🧒", "Детям"),
    ("🏛", "Политика"),
    ("🎬", "Фильмы и сериалы"),
    ("🎵", "Музыка"),
    ("📺", "Шоу"),
];

pub fn show(ui: &mut egui::Ui, active: &str) -> Option<Nav> {
    let mut nav = None;
    ui.add_space(8.0);

    for group in [MAIN, LIBRARY, CATS] {
        for &(icon, label) in group {
            if item(ui, icon, label, active) {
                nav = Some(if label == "Главная" {
                    Nav::Home
                } else {
                    Nav::Query(label.to_string())
                });
            }
        }
        ui.add_space(8.0);
        if !group.is_empty() && group != CATS {
            ui.separator();
            ui.add_space(8.0);
        }
    }

    nav
}

fn item(ui: &mut egui::Ui, icon: &str, label: &str, active: &str) -> bool {
    let is_active = active == label;
    
    let (rect, resp) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), 40.0),
        egui::Sense::click(),
    );
    
    // Фон при наведении и активном состоянии
    if resp.hovered() || is_active {
        ui.painter().rect_filled(
            rect.shrink(4.0),
            egui::Rounding::same(10.0),
            if is_active { BG_HOVER } else { BG_HOVER },
        );
    }
    
    // Иконка с акцентным цветом для активного элемента
    let text_color = if is_active { ACCENT } else { TEXT_SECONDARY };
    let icon_color = if is_active { ACCENT } else { TEXT_MUTED };
    
    // Рисуем иконку
    let icon_rect = egui::Rect::from_min_size(
        rect.min + egui::vec2(16.0, 8.0),
        egui::vec2(24.0, 24.0),
    );
    ui.painter().text(
        icon_rect.center(),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(18.0),
        icon_color,
    );
    
    // Рисуем текст
    let text_rect = egui::Rect::from_min_size(
        rect.min + egui::vec2(48.0, 0.0),
        egui::vec2(rect.width() - 56.0, 40.0),
    );
    ui.painter().text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::proportional(14.0).weight(
            if is_active { egui::FontWeight::MEDIUM } else { egui::FontWeight::NORMAL }
        ),
        text_color,
    );
    
    resp.clicked()
}
