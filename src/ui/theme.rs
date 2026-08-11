// theme.rs
use eframe::egui;

pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(0, 119, 255);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
pub const GRAY: egui::Color32 = egui::Color32::from_rgb(129, 140, 153);
pub const BG: egui::Color32 = egui::Color32::WHITE;
pub const PANEL: egui::Color32 = egui::Color32::from_rgb(240, 242, 245);
pub const HOVER: egui::Color32 = egui::Color32::from_rgb(228, 231, 235);
pub const CHIP_ACTIVE_BG: egui::Color32 = egui::Color32::from_rgb(232, 242, 255);

pub fn apply(ctx: &egui::Context) {
    let mut v = egui::Visuals::light();
    v.panel_fill = BG;
    v.window_fill = BG;
    v.extreme_bg_color = BG;
    v.widgets.noninteractive.bg_fill = BG;
    v.widgets.inactive.weak_bg_fill = BG;
    v.selection.bg_fill = ACCENT;
    v.window_rounding = egui::Rounding::same(12.0);
    v.menu_rounding = egui::Rounding::same(8.0);
    ctx.set_visuals(v);
}

/// Фрейм панели без тени и рамки (иначе egui рисует чёрную тень под панелью)
pub fn panel_frame(margin: f32) -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(margin),
        outer_margin: egui::Margin::same(0.0),
        rounding: egui::Rounding::same(0.0),
        shadow: egui::Shadow::NONE,
        fill: BG,
        stroke: egui::Stroke::NONE,
    }
}
