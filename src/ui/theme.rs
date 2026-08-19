// theme.rs
use eframe::egui;

// Современная тёмная палитра
pub const BG_PRIMARY: egui::Color32 = egui::Color32::from_rgb(15, 15, 20);
pub const BG_SECONDARY: egui::Color32 = egui::Color32::from_rgb(25, 25, 35);
pub const BG_HOVER: egui::Color32 = egui::Color32::from_rgb(35, 35, 50);
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(0, 132, 255);
pub const ACCENT_GRADIENT_START: egui::Color32 = egui::Color32::from_rgb(0, 132, 255);
pub const ACCENT_GRADIENT_END: egui::Color32 = egui::Color32::from_rgb(0, 90, 200);
pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(160, 160, 170);
pub const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(100, 100, 110);
pub const DIVIDER: egui::Color32 = egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20);
pub const CHIP_BG: egui::Color32 = egui::Color32::from_rgb(35, 35, 45);
pub const CHIP_ACTIVE_BG: egui::Color32 = egui::Color32::from_rgb(25, 25, 35);
pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(25, 25, 35);
pub const SHADOW_COLOR: egui::Color32 = egui::Color32::from_black_alpha(80);
pub const PLAY_BUTTON_BG: egui::Color32 = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180);

pub fn apply(ctx: &egui::Context) {
    // Настройка шрифтов для лучшего отображения текста
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::proportional(14.0),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::proportional(12.0),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::proportional(20.0),
    );
    ctx.set_style(style);
    
    let mut v = egui::Visuals::dark();
    v.panel_fill = BG_PRIMARY;
    v.window_fill = BG_SECONDARY;
    v.extreme_bg_color = BG_PRIMARY;
    v.widgets.noninteractive.bg_fill = BG_SECONDARY;
    v.widgets.inactive.weak_bg_fill = BG_SECONDARY;
    v.widgets.hovered.weak_bg_fill = BG_HOVER;
    v.widgets.active.weak_bg_fill = BG_HOVER;
    v.selection.bg_fill = ACCENT;
    v.window_rounding = egui::Rounding::same(16.0);
    v.menu_rounding = egui::Rounding::same(12.0);
    v.popup_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 20.0,
        spread: 0.0,
        color: SHADOW_COLOR,
    };
    v.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 8.0),
        blur: 32.0,
        spread: 0.0,
        color: SHADOW_COLOR,
    };
    ctx.set_visuals(v);
}

/// Фрейм панели без тени и рамки
pub fn panel_frame(margin: f32) -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(margin),
        outer_margin: egui::Margin::same(0.0),
        rounding: egui::Rounding::same(0.0),
        shadow: egui::Shadow::NONE,
        fill: BG_PRIMARY,
        stroke: egui::Stroke::NONE,
    }
}

/// Фрейм для карточек с тенью
pub fn card_frame() -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(0.0),
        outer_margin: egui::Margin::same(0.0),
        rounding: egui::Rounding::same(12.0),
        shadow: egui::Shadow {
            offset: egui::vec2(0.0, 2.0),
            blur: 8.0,
            spread: 0.0,
            color: SHADOW_COLOR,
        },
        fill: CARD_BG,
        stroke: egui::Stroke::new(1.0, DIVIDER),
    }
}

/// Градиентный акцент
pub fn paint_accent_gradient(painter: &egui::Painter, rect: egui::Rect, rounding: egui::Rounding) {
    painter.rect_filled(rect, rounding, ACCENT_GRADIENT_START);
    
    // Создаём эффект градиента вручную
    let gradient_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width(), rect.height() * 0.3),
    );
    painter.rect_filled(gradient_rect, rounding, ACCENT_GRADIENT_END);
}
