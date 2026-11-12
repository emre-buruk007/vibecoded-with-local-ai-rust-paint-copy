//! PaintApp: the main application struct with eframe::App implementation.

use eframe::egui;
use eframe::App;
use crate::canvas::Canvas;
use crate::tool::Tool;
use crate::history::UndoStack;
use crate::draw::DrawState;
use crate::shapes;
use crate::shapes::Point;

#[derive(Clone, Copy, Debug)]
struct CanvasLayout {
    rect: egui::Rect,
    scale: f32,
}

impl Default for CanvasLayout {
    fn default() -> Self {
        Self {
            rect: egui::Rect::NOTHING,
            scale: 1.0,
        }
    }
}

pub struct PaintApp {
    canvas: Canvas,
    tool: Tool,
    color: egui::Color32,
    brush_size: f32,
    fill_shape: bool,
    draw_state: DrawState,
    undo_stack: UndoStack,
    pointer_pos: Option<egui::Pos2>,
    primary_just_pressed: bool,
    primary_just_released: bool,
    show_about: bool,
    show_resize: bool,
    resize_w: String,
    resize_h: String,
    canvas_layout: CanvasLayout,
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
            primary_just_pressed: false,
            primary_just_released: false,
            show_about: false,
            show_resize: false,
            resize_w: Canvas::CW.to_string(),
            resize_h: Canvas::CH.to_string(),
            canvas_layout: CanvasLayout::default(),
        }
    }

    fn save_state(&mut self) {
        self.undo_stack.push(self.canvas.clone_data());
    }

    fn undo(&mut self) {
        let current = self.canvas.clone_data();
        if let Some(data) = self.undo_stack.pop(&current) {
            self.canvas.load_from_pixels(&data);
        }
    }

    fn redo(&mut self) {
        let current = self.canvas.clone_data();
        if let Some(data) = self.undo_stack.redo_pop(&current) {
            self.canvas.load_from_pixels(&data);
        }
    }

    fn clear_canvas(&mut self) {
        self.save_state();
        self.canvas.fill(egui::Color32::WHITE);
    }

    fn canvas_to_color_image(&self) -> egui::ColorImage {
        let cw = self.canvas.width() as usize;
        let ch = self.canvas.height() as usize;
        let mut image = egui::ColorImage::new([cw, ch], egui::Color32::TRANSPARENT);
        for y in 0..ch {
            for x in 0..cw {
                if let Some(pixel) = self.canvas.get_pixel(x as u32, y as u32) {
                    let idx = y * cw + x;
                    image.pixels[idx] = egui::Color32::from_rgba_unmultiplied(pixel[0], pixel[1], pixel[2], pixel[3]);
                }
            }
        }
        image
    }

    fn screen_to_canvas(&self, screen_pos: egui::Pos2) -> Option<Point> {
        let layout = self.canvas_layout;
        let local = screen_pos - layout.rect.min;
        let canvas_x = (local.x / layout.scale) as i32;
        let canvas_y = (local.y / layout.scale) as i32;
        let cw = self.canvas.width() as i32;
        let ch = self.canvas.height() as i32;
        if canvas_x < 0 || canvas_x >= cw || canvas_y < 0 || canvas_y >= ch {
            None
        } else {
            Some(Point::new(canvas_x, canvas_y))
        }
    }
}

impl App for PaintApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.pointer_pos = ctx.pointer_hover_pos();

        // MENU BAR
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open...").clicked() {}
                    if ui.button("Save...").clicked() {}
                    ui.separator();
                    if ui.button("Resize Canvas...").clicked() {
                        self.show_resize = true;
                    }
                    ui.separator();
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo  (Ctrl+Z)").clicked() {
                        self.undo();
                    }
                    if ui.button("Redo  (Ctrl+Y)").clicked() {
                        self.redo();
                    }
                    if ui.button("Clear Canvas").clicked() {
                        self.clear_canvas();
                    }
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} x {}", Canvas::CW, Canvas::CH));
                });
            });
        });

        // LEFT TOOLBAR
        let toolbar_actions = egui::SidePanel::left("toolbar")
            .default_width(180.0)
            .show(ctx, |ui| {
                crate::ui::toolbar::toolbar_ui(
                    ui, &mut self.tool, &mut self.color,
                    &mut self.brush_size, &mut self.fill_shape,
                    self.undo_stack.undo_len() > 0,
                    self.undo_stack.redo_len() > 0,
                )
            });
        let actions = toolbar_actions.inner;
        if actions.undo { self.undo(); }
        if actions.redo { self.redo(); }
        if actions.clear { self.clear_canvas(); }
        if actions.about { self.show_about = true; }

        // RIGHT PANEL
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();
                ui.label("1 layer");
            });

        // CENTRAL CANVAS
        egui::CentralPanel::default().show(ctx, |ui| {
            self.primary_just_pressed = ui.input(|i| i.pointer.primary_pressed());
            self.primary_just_released = ui.input(|i| i.pointer.primary_released());

            let available = ui.available_size();
            let cw_f = Canvas::CW as f32;
            let ch_f = Canvas::CH as f32;
            let aspect = cw_f / ch_f;
            let (canvas_w, canvas_h) = if available.x / available.y > aspect {
                let h = available.y;
                (h * aspect, h)
            } else {
                (available.x, available.x / aspect)
            };
            let canvas_w = canvas_w.floor().max(100.0);
            let canvas_h = canvas_h.floor().max(100.0);
            let size = egui::Vec2::new(canvas_w, canvas_h);
            let scale = canvas_w / cw_f;

            let (alloc_rect, _resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
            self.canvas_layout = CanvasLayout { rect: alloc_rect, scale };

            let tile_size = 16.0;
            for x in 0..(canvas_w / tile_size + 1.0) as i32 {
                for y in 0..(canvas_h / tile_size + 1.0) as i32 {
                    let is_white = (x + y) % 2 == 0;
                    let tc = if is_white { egui::Color32::GRAY } else { egui::Color32::DARK_GRAY };
                    let tr = egui::Rect::from_min_size(
                        alloc_rect.left_top() + egui::vec2(x as f32 * tile_size, y as f32 * tile_size),
                        egui::Vec2::splat(tile_size),
                    );
                    ui.painter().rect_filled(tr, 0.0, tc);
                }
            }

            let color_image = self.canvas_to_color_image();
            let texture_id = ui.ctx().load_texture("canvas", color_image, egui::TextureOptions::default()).id();
            ui.painter().image(texture_id, alloc_rect, egui::Rect::EVERYTHING, egui::Color32::WHITE);

            let tool_color = if self.tool == Tool::Eraser { egui::Color32::WHITE } else { self.color };

            // PRESS
            if self.primary_just_pressed {
                if let Some(pos) = self.pointer_pos {
                    if let Some(pt) = self.screen_to_canvas(pos) {
                        // Cancel any active draw state before starting a new one (prevents rapid-click state corruption)
                        if !matches!(self.draw_state, DrawState::None) {
                            self.draw_state = DrawState::None;
                        }

                        if self.tool == Tool::Pencil || self.tool == Tool::Eraser {
                            // Pencil/Eraser: draw immediately to canvas buffer
                            self.save_state();
                            let r = (self.brush_size / 2.0) as i32;
                            for dy in -r..=r {
                                for dx in -r..=r {
                                    if dx * dx + dy * dy <= r * r {
                                        // Use set_pixel (clamps to bounds) instead of set_pixel_unchecked to prevent out-of-bounds panic
                                        self.canvas.set_pixel((pt.x + dx) as u32, (pt.y + dy) as u32, tool_color);
                                    }
                                }
                            }
                            self.draw_state = DrawState::new(pt);
                        } else if self.tool.is_shape() {
                            // Shape tools: just record start, preview drawn via Painter overlay
                            self.draw_state = DrawState::new(pt);
                        }
                        ctx.request_repaint();
                    }
                }
            }

            // DRAG (pencil/eraser draw to buffer; shapes draw dashed preview via Painter overlay)
            if matches!(self.draw_state, DrawState::Drawing { .. }) {
                if let Some(pos) = self.pointer_pos {
                    let layout = self.canvas_layout;
                    let canvas_pt = self.screen_to_canvas(pos);
                    if let Some(pt) = canvas_pt {
                        let start_pt = match self.draw_state {
                            DrawState::Drawing { start } => start,
                            _ => pt,
                        };

                        match self.tool {
                            // Pencil and eraser: draw directly to canvas buffer (permanent)
                            Tool::Pencil | Tool::Eraser => {
                                shapes::draw_line(&mut self.canvas, start_pt, pt, tool_color);
                                ctx.request_repaint();
                            }
                            // Line tool: dashed preview via Painter overlay
                            Tool::Line => {
                                let start_screen = egui::pos2(
                                    (start_pt.x as f32) * layout.scale + layout.rect.min.x,
                                    (start_pt.y as f32) * layout.scale + layout.rect.min.y,
                                );
                                let end_screen = egui::pos2(
                                    (pt.x as f32) * layout.scale + layout.rect.min.x,
                                    (pt.y as f32) * layout.scale + layout.rect.min.y,
                                );
                                ui.painter().line_segment([start_screen, end_screen], (1.5, tool_color));
                                ctx.request_repaint();
                            }
                            // Rectangle: dashed preview via Painter overlay
                            Tool::Rectangle => {
                                let start_screen = egui::pos2(
                                    (start_pt.x as f32) * layout.scale + layout.rect.min.x,
                                    (start_pt.y as f32) * layout.scale + layout.rect.min.y,
                                );
                                let end_screen = egui::pos2(
                                    (pt.x as f32) * layout.scale + layout.rect.min.x,
                                    (pt.y as f32) * layout.scale + layout.rect.min.y,
                                );
                                let rect = egui::Rect::from_two_pos(start_screen, end_screen);
                                if self.fill_shape {
                                    ui.painter().rect_filled(rect, 0.0, tool_color);
                                } else {
                                    ui.painter().rect_stroke(rect, 0.0, (1.5, tool_color));
                                }
                                ctx.request_repaint();
                            }
                            // Ellipse: preview via Painter overlay (bounding box rect)
                            Tool::Ellipse => {
                                let cx = (start_pt.x + pt.x) / 2;
                                let cy = (start_pt.y + pt.y) / 2;
                                let rx = (pt.x - start_pt.x).abs() / 2;
                                let ry = (pt.y - start_pt.y).abs() / 2;
                                let center_screen = egui::pos2(
                                    (cx as f32) * layout.scale + layout.rect.min.x,
                                    (cy as f32) * layout.scale + layout.rect.min.y,
                                );
                                let radii = egui::vec2(rx as f32, ry as f32);
                                if self.fill_shape {
                                    ui.painter().circle(center_screen, radii.x, tool_color, egui::Stroke::NONE);
                                } else {
                                    ui.painter().circle(center_screen, radii.x, egui::Color32::TRANSPARENT, egui::Stroke::new(1.5_f32, tool_color));
                                }
                                ctx.request_repaint();
                            }
                            // Fill and eyedropper: no drag behavior
                            Tool::Fill | Tool::Eyedropper => {}
                        }
                    }
                }
            }

            // RELEASE (finalize shape to canvas buffer)
            if self.primary_just_released && matches!(self.draw_state, DrawState::Drawing { .. }) {
                if let DrawState::Drawing { start } = self.draw_state {
                    let start_pt = start;
                    // Fix 4: Guard - skip if canvas dimensions changed (e.g., after resize) which could invalidate the start point
                    let cw = self.canvas.width() as i32;
                    let ch = self.canvas.height() as i32;
                    if start_pt.x < 0 || start_pt.x >= cw || start_pt.y < 0 || start_pt.y >= ch {
                        self.draw_state = DrawState::None;
                        ctx.request_repaint();
                        return;
                    }
                    if let Some(end_pt) = self.pointer_pos.and_then(|p| self.screen_to_canvas(p)) {
                        let dx = (start_pt.x as f32 - end_pt.x as f32).abs();
                        let dy = (start_pt.y as f32 - end_pt.y as f32).abs();
                        let dist = dx.hypot(dy);
                        match self.tool {
                            Tool::Line => {
                                if dist > 1.0 {
                                    shapes::draw_line(&mut self.canvas, start_pt, end_pt, tool_color);
                                }
                            }
                            Tool::Rectangle => {
                                if dist > 1.0 {
                                    if self.fill_shape {
                                        shapes::fill_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                                    } else {
                                        shapes::draw_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                                    }
                                }
                            }
                            Tool::Ellipse => {
                                if dist > 1.0 {
                                    let cx = (start_pt.x + end_pt.x) / 2;
                                    let cy = (start_pt.y + end_pt.y) / 2;
                                    let rx = (end_pt.x - start_pt.x).abs() / 2;
                                    let ry = (end_pt.y - start_pt.y).abs() / 2;
                                    let center = Point::new(cx, cy);
                                    if self.fill_shape {
                                        shapes::fill_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                                    } else {
                                        shapes::draw_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    self.draw_state = DrawState::None;
                    ctx.request_repaint();
                }
            }
        });

        if self.show_resize {
            egui::Window::new("Resize Canvas")
                .collapsible(false).resizable(false)
                .show(ctx, |ui| {
                    ui.label("Width (px):");
                    ui.text_edit_singleline(&mut self.resize_w);
                    ui.label("Height (px):");
                    ui.text_edit_singleline(&mut self.resize_h);
                    ui.separator();
                    let w = self.resize_w.parse::<u32>().unwrap_or(Canvas::CW);
                    let h = self.resize_h.parse::<u32>().unwrap_or(Canvas::CH);
                    if ui.button("Resize").clicked() && w >= 10 && h >= 10 {
                        let old_data = self.canvas.clone_data();
                        self.canvas.resize(w, h);
                        self.canvas.load_from_pixels(&old_data);
                        self.show_resize = false;
                        ctx.request_repaint();
                    }
                    ui.separator();
                    if ui.button("Cancel").clicked() {
                        self.show_resize = false;
                    }
                });
        }

        if self.show_about {
            egui::Window::new("About Paint")
                .collapsible(false).resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Rust Paint v1.0");
                    ui.label("A simple painting app built with egui and eframe.");
                    ui.separator();
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }
    }
}
