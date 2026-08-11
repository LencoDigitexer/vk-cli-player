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
    ui.add_space(4.0);

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
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);
    }

    nav
}

fn item(ui: &mut egui::Ui, icon: &str, label: &str, active: &str) -> bool {
    let is_active = active == label;
    let btn = egui::Button::new(
        egui::RichText::new(format!("{}   {}", icon, label))
            .size(14.0)
            .color(TEXT),
    )
    .fill(if is_active {
        HOVER
    } else {
        egui::Color32::TRANSPARENT
    })
    .rounding(egui::Rounding::same(8.0));

    ui.add_sized([ui.available_width(), 36.0], btn).clicked()
}
