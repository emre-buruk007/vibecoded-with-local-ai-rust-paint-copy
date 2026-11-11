//! Left toolbar with tool buttons, color picker, and brush size slider.

use eframe::egui;
use crate::tool::Tool;

pub const PALETTE: [egui::Color32; 12] = [
    egui::Color32::BLACK,
    egui::Color32::WHITE,
    egui::Color32::RED,
    egui::Color32::GREEN,
    egui::Color32::BLUE,
    egui::Color32::YELLOW,
    egui::Color32::from_rgb(255, 0, 255),
    egui::Color32::from_rgb(0, 255, 255),
    egui::Color32::DARK_GRAY,
    egui::Color32::GRAY,
    egui::Color32::LIGHT_GRAY,
    egui::Color32::ORANGE,
];

pub fn toolbar_ui(ui: &mut egui::Ui, tool: &mut Tool, color: &mut egui::Color32, brush_size: &mut f32, fill_shape: &mut bool) {
    ui.vertical(|ui| {
        ui.heading("\u{1F3A8} Paint");
        ui.separator();

        // Tool buttons
        for t in Tool::all() {
            let is_shape = matches!(t, Tool::Rectangle | Tool::Ellipse);

            if ui.selectable_label(*tool == t, t.name()).clicked() {
                *tool = t;
            }

            // Show fill toggle only for shape tools
            if is_shape {
                ui.checkbox(fill_shape, "Fill");
            }
        }

        ui.separator();

        // Brush size
        ui.label("Brush Size");
        ui.add(egui::Slider::new(brush_size, 1.0..=50.0).text("Size"));

        ui.separator();

        // Color picker
        ui.label("Color");
        for c in &PALETTE {
            let mut clone = *c;
            if ui.color_edit_button_srgba(&mut clone).changed() {
                if clone != *c {
                    *color = clone;
                }
            }
        }
        // Custom color picker
        ui.color_edit_button_srgba(color);

        ui.separator();

        // Grid toggle
        ui.checkbox(&mut false, "Show Grid");

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("\u{21B5} Undo").clicked() { /* handled in app */ }
            if ui.button("\u{21AA} Redo").clicked() { /* handled in app */ }
        });

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("\u{1F4C2} Open").clicked() { /* handled in app */ }
            if ui.button("\u{1F4BE} Save").clicked() { /* handled in app */ }
        });

        ui.separator();
        if ui.button("About").clicked() { /* handled in app */ }
        if ui.button("Clear").clicked() { /* handled in app */ }
    });
}
