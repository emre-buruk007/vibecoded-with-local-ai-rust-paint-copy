//! Left toolbar with tool buttons, color picker, and brush size slider.

use eframe::egui;
use crate::tool::Tool;

/// Actions requested by the toolbar buttons.
#[derive(Default)]
pub struct ToolbarActions {
    pub undo: bool,
    pub redo: bool,
    pub clear: bool,
    pub about: bool,
}

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

/// Color preview swatch size
const COLOR_SWATCH_SIZE: f32 = 28.0;

pub fn toolbar_ui(
    ui: &mut egui::Ui,
    tool: &mut Tool,
    color: &mut egui::Color32,
    brush_size: &mut f32,
    fill_shape: &mut bool,
    undo_available: bool,
    redo_available: bool,
) -> ToolbarActions {
    let mut actions = ToolbarActions::default();
    ui.vertical(|ui| {
        ui.heading("\u{1F3A8} Paint");
        ui.separator();

        // Tool buttons
        for t in Tool::all() {
            ui.horizontal(|ui| {
                let selected = ui.selectable_label(*tool == t, t.name());
                if selected.clicked() {
                    *tool = t;
                }
                // Only show fill checkbox for shape tools in the menu
            });
        }

        // Single fill checkbox (shown when a shape tool is selected)
        if tool.is_shape() {
            ui.checkbox(fill_shape, "Fill");
        }

        ui.separator();

        // Brush size
        ui.label("Brush Size");
        ui.add(egui::Slider::new(brush_size, 1.0..=50.0).text("Size"));

        ui.separator();

        // Color preview + picker
        ui.label("Color");

        // Color preview swatch
        let preview_size = egui::Vec2::splat(COLOR_SWATCH_SIZE);
        ui.horizontal(|ui| {
            let frame = egui::Frame::none().fill(*color).stroke(
                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(100, 100, 120)),
            );
            frame
                .show(ui, |ui| {
                    ui.allocate_ui_with_layout(preview_size, egui::Layout::centered_and_justified(egui::Direction::TopDown), |_ui| {
                        // Empty interior
                    });
                })
                .response
                .on_hover_text(format!("Current color: R={}, G={}, B={}, A={}", color[0], color[1], color[2], color[3]));

            // Color edit button (opens color picker dialog)
            if ui.color_edit_button_srgba(color).clicked() {
                // egui shows its built-in color picker popup
            }
        });

        // Palette swatches
        ui.label("Palette");
        ui.horizontal_wrapped(|ui| {
            for c in &PALETTE {
                let btn = egui::Button::new("")
                    .fill(*c)
                    .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(80, 80, 100)))
                    .min_size(egui::Vec2::splat(20.0));
                if ui.add(btn).clicked() {
                    *color = *c;
                }
            }
        });

        ui.separator();

        // Undo / Redo
        ui.horizontal(|ui| {
            if ui.button("↶ Undo").clicked() && undo_available {
                actions.undo = true;
            }
            if ui.button("↷ Redo").clicked() && redo_available {
                actions.redo = true;
            }
        });

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("📂 Open").clicked() {
                // TODO: handled via file dialog
            }
            if ui.button("💾 Save").clicked() {
                // TODO: handled via file dialog
            }
        });

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("🗑 Clear").clicked() {
                actions.clear = true;
            }
        });

        ui.separator();
        if ui.button("ℹ About").clicked() {
            actions.about = true;
        }
    });
    actions
}
