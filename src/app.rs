//! PaintApp: the main application struct with eframe::App implementation.

use eframe::egui;
use eframe::App;
use crate::canvas::Canvas;
use crate::tool::Tool;
use crate::history::UndoStack;
use crate::draw::DrawState;
use crate::shapes;
use crate::shapes::Point;

pub struct PaintApp {
    canvas: Canvas,
    tool: Tool,
    color: egui::Color32,
    brush_size: f32,
    fill_shape: bool,
    draw_state: DrawState,
    undo_stack: UndoStack,
    /// Tracked pointer position for drawing when interact_pos() is unreliable.
    pointer_pos: Option<egui::Pos2>,
}

impl PaintApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        PaintApp {
            canvas: Canvas::new(),
            tool: Tool::Pencil,
            color: egui::Color32::BLACK,
            brush_size: 3.0,
            fill_shape: false,
            draw_state: DrawState::None,
            undo_stack: UndoStack::new(20),
            pointer_pos: None,
        }
    }

    fn save_state(&mut self) {
        self.undo_stack.push(self.canvas.clone_data());
    }

    fn undo(&mut self) {
        if let Some(data) = self.undo_stack.pop() {
            self.canvas.load_from_pixels(&data);
        }
    }
    fn apply_tool(canvas: &mut Canvas, tool: Tool, color: egui::Color32, brush_size: f32, fill_shape: bool, draw_state: &DrawState, screen_pos: egui::Pos2) {
        let tool_color = if tool == Tool::Eraser { egui::Color32::WHITE } else { color };
        let size = brush_size as i32;
        match (tool, draw_state) {
            (Tool::Pencil | Tool::Eraser, DrawState::Drawing { .. }) => {
                let end = Point::new(screen_pos.x as i32, screen_pos.y as i32);
                let start_pos = draw_state.start_pos();
                let start = Point::new(start_pos.x as i32, start_pos.y as i32);
                shapes::draw_line(canvas, start, end, tool_color);
                for dy in -size..=size {
                    for dx in -size..=size {
                        if dx*dx + dy*dy <= size*size {
                            let px = (end.x + dx).max(0) as u32;
                            let py = (end.y + dy).max(0) as u32;
                            if px < Canvas::CW && py < Canvas::CH {
                                canvas.set_pixel_unchecked(px, py, tool_color);
                            }
                        }
                    }
                }
            }
            (Tool::Line, DrawState::Drawing { .. }) => {
                let end = Point::new(screen_pos.x as i32, screen_pos.y as i32);
                let start_pos = draw_state.start_pos();
                let start = Point::new(start_pos.x as i32, start_pos.y as i32);
                shapes::draw_line(canvas, start, end, tool_color);
            }
            (Tool::Rectangle, DrawState::Drawing { .. }) => {
                let end = Point::new(screen_pos.x as i32, screen_pos.y as i32);
                let start_pos = draw_state.start_pos();
                let start = Point::new(start_pos.x as i32, start_pos.y as i32);
                if fill_shape { shapes::fill_rect(canvas, start, end, tool_color); }
                else { shapes::draw_rect(canvas, start, end, tool_color); }
            }
            (Tool::Ellipse, DrawState::Drawing { .. }) => {
                let end = Point::new(screen_pos.x as i32, screen_pos.y as i32);
                let start_pos = draw_state.start_pos();
                let start = Point::new(start_pos.x as i32, start_pos.y as i32);
                let center = Point::new((start.x + end.x) / 2, (start.y + end.y) / 2);
                let rx = (end.x - start.x).abs() / 2;
                let ry = (end.y - start.y).abs() / 2;
                if fill_shape { shapes::fill_ellipse(canvas, center, rx, ry, tool_color); }
                else { shapes::draw_ellipse(canvas, center, rx, ry, tool_color); }
            }
            _ => {}
        }
    }

    fn canvas_to_image(&self) -> egui::ColorImage {
        let mut img = egui::ColorImage::new([Canvas::CW as usize, Canvas::CH as usize], egui::Color32::TRANSPARENT);
        for y in 0..Canvas::CH {
            for x in 0..Canvas::CW {
                if let Some(c) = self.canvas.get_pixel(x, y) {
                    let idx = (y * Canvas::CW + x) as usize;
                    if idx < img.pixels.len() {
                        img.pixels[idx] = c;
                    }
                }
            }
        }
        img
    }

    /// Convert egui ColorImage to a vector of RGBA bytes for saving.
    fn image_to_bytes(img: &egui::ColorImage) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(img.width() * img.height() * 4);
        for pixel in &img.pixels {
            bytes.push(pixel[0]);
            bytes.push(pixel[1]);
            bytes.push(pixel[2]);
            bytes.push(pixel[3]);
        }
        bytes
    }
}

impl App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply dark theme
        ctx.style_mut(|style| {
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(20, 20, 30);
            style.visuals.window_fill = egui::Color32::from_rgb(30, 30, 45);
            style.visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(50, 50, 70));
            style.visuals.panel_fill = egui::Color32::from_rgb(25, 25, 40);
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 35, 55);
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 55);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 70);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
            style.visuals.widgets.active.rounding = egui::Rounding::same(4.0);
            style.visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
            style.visuals.menu_rounding = egui::Rounding::same(6.0);
            style.visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 240));
            style.spacing.button_padding = egui::vec2(8.0, 4.0);
            style.spacing.item_spacing = egui::vec2(4.0, 4.0);
        });

        // Menu bar
        egui::TopBottomPanel::top("menu_bar").resizable(false).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Canvas").clicked() {
                        self.canvas = Canvas::new();
                        self.undo_stack = UndoStack::new(20);
                    }
                    if ui.button("Clear Canvas").clicked() {
                        self.save_state();
                        self.canvas = Canvas::new();
                    }
                    ui.separator();
                    if ui.button("Save as PNG").clicked() {
                        let img = self.canvas_to_image();
                        let bytes = Self::image_to_bytes(&img);
                        let image = image::DynamicImage::ImageRgba8(image::RgbaImage::from_vec(Canvas::CW, Canvas::CH, bytes).expect("Invalid image buffer"));
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("PNG", &["png"])
                            .set_file_name("painting.png")
                            .save_file()
                        {
                            if let Err(e) = image.save(&path) {
                                eprintln!("Failed to save image: {}", e);
                            }
                        }
                    }
                    if ui.button("Open").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Image", &["png", "jpg", "jpeg"])
                            .set_file_name("image.png")
                            .pick_file()
                        {
                            if let Ok(img) = image::open(&path) {
                                let bgr = img.to_rgba8();
                                self.save_state();
                                self.canvas.load_from_pixels(&bgr);
                            }
                        }
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        self.undo();
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                });
            });
        });
        // Left toolbar
        egui::SidePanel::left("toolbar").resizable(false).show(ctx, |ui| {
            let mut current_tool = self.tool;
            let mut current_fill = self.fill_shape;
            let mut current_color = self.color;
            let current_brush = self.brush_size;

            ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_rgb(220, 220, 240));
            ui.style_mut().visuals.widgets.active.bg_fill = egui::Color32::from_rgb(59, 130, 246);
            ui.style_mut().visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 70);
            ui.style_mut().visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(35, 35, 55);

            ui.heading("Paint");
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            for t in Tool::all() {
                let is_shape = t.is_shape();
                let selected = current_tool == t;
                if ui.selectable_label(selected, format!("{} {}", t.icon(), t.name())).clicked() {
                    self.tool = t;
                    current_tool = t;
                }
                ui.add_space(2.0);
                if is_shape {
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        let mut fill_clone = current_fill;
                        if ui.checkbox(&mut fill_clone, "Fill").clicked() {
                            self.fill_shape = fill_clone;
                            current_fill = fill_clone;
                        }
                    });
                }
                ui.add_space(6.0);
            }

            ui.separator();
            ui.add_space(4.0);
            ui.label("Brush Size");
            let mut slider_val = current_brush;
            if ui.add(egui::Slider::new(&mut slider_val, 1.0..=50.0).text("Size")).changed() {
                self.brush_size = slider_val;
                
            }
            ui.add_space(6.0);

            ui.separator();
            ui.add_space(4.0);
            ui.label("Color");
            for c in &crate::ui::toolbar::PALETTE {
                let mut clone = *c;
                if ui.color_edit_button_srgba(&mut clone).changed() {
                    self.color = clone;
                    current_color = clone;
                }
            }
            ui.add_space(2.0);
            if ui.color_edit_button_srgba(&mut current_color).changed() {
                self.color = current_color;
            }

            ui.add_space(12.0);
        });
        // Bottom info bar
        egui::TopBottomPanel::bottom("info_bar").resizable(false).show(ctx, |ui| {
            ui.style_mut().visuals.override_text_color = Some(egui::Color32::from_rgb(160, 160, 190));
            ui.horizontal(|ui| {
                ui.label(format!("Tool: {} ({}) - {}x{}px", self.tool.name(), self.tool.icon(), Canvas::CW, Canvas::CH));
                ui.separator();
                ui.label(format!("Brush: {:.0}px", self.brush_size));
                ui.separator();
                ui.label(format!("Color: #{:02X}{:02X}{:02X}", self.color[0], self.color[1], self.color[2]));
                ui.separator();
                if let Some(p) = self.pointer_pos {
                    ui.label(format!("Cursor: ({:.0}, {:.0})", p.x, p.y));
                }
            });
        });

        // Central canvas area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 40, 55);

            let canvas_rect = ui.available_rect_before_wrap();
            let aspect = Canvas::CW as f32 / Canvas::CH as f32;
            let (tex_w, tex_h) = if canvas_rect.width() / canvas_rect.height() > aspect {
                (canvas_rect.height() * aspect, canvas_rect.height())
            } else {
                (canvas_rect.width(), canvas_rect.width() / aspect)
            };
            let texture_rect = egui::Rect::from_min_size(
                egui::Pos2::new(canvas_rect.min.x, canvas_rect.min.y),
                egui::Vec2::new(tex_w, tex_h),
            );
            // Draw canvas background with border
            ui.painter().rect_filled(
                texture_rect,
                4.0,
                egui::Color32::from_rgb(255, 255, 255),
            );
            ui.painter().rect_stroke(
                texture_rect,
                4.0,
                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(100, 100, 130)),
            );

            let img = self.canvas_to_image();
            let handle = ctx.load_texture("canvas", img, egui::TextureOptions::default());
            ui.painter().image(
                handle.id(),
                texture_rect,
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(Canvas::CW as f32, Canvas::CH as f32)),
                egui::Color32::WHITE,
            );
            // === POINTER POSITION TRACKING ===
            // Track pointer position manually because egui 0.29 PointerState
            // does not expose a direct `pos()` field. We detect the start of
            // interaction via `is_using()` transitions.
            let was_using = self.pointer_pos.is_some();
            let primary_pressed = ui.input(|i| i.pointer.primary_pressed());
            if primary_pressed {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    self.pointer_pos = Some(pos);
                }
            } else if !was_using && self.pointer_pos.is_none() {
                // Reset on pointer release
                self.pointer_pos = None;
            }

            // === DRAWING LOGIC ===
            if ui.input(|i| i.pointer.primary_down()) {
                if let Some(pos) = self.pointer_pos {
                    match self.draw_state {
                        DrawState::None => {
                            // --- Initial click: start a new drawing operation ---
                            self.save_state();

                            // Fill tool: flood fill immediately on single click
                            if self.tool == Tool::Fill {
                                let pt = Point::new(pos.x as i32, pos.y as i32);
                                shapes::flood_fill(&mut self.canvas, pt, self.color, 32);
                            }
                            // Eyedropper tool: pick color on single click
                            else if self.tool == Tool::Eyedropper {
                                let pt = Point::new(pos.x as i32, pos.y as i32);
                                if let Some(c) = self.canvas.get_pixel(pt.x as u32, pt.y as u32) {
                                    self.color = c;
                                }
                            }
                            // All other tools: start tracking for drag drawing
                            else {
                                self.draw_state = DrawState::new(pos);
                            }
                        }
                        DrawState::Drawing { .. } => {
                            // --- Pointer is dragging: continue drawing ---
                            Self::apply_tool(
                                &mut self.canvas,
                                self.tool,
                                self.color,
                                self.brush_size,
                                self.fill_shape,
                                &self.draw_state,
                                pos,
                            );
                        }
                    }
                }
            } else if matches!(self.draw_state, DrawState::Drawing { .. }) {
                // Pointer released: clear the drawing state
                self.draw_state = DrawState::None;
            }

            ui.expand_to_include_rect(texture_rect);
            ui.allocate_rect(texture_rect, egui::Sense::click_and_drag());
        });

        ctx.request_repaint();
    }
}


