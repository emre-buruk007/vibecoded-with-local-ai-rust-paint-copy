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
                if ui.button("File").clicked() {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open...").clicked() {}
                        if ui.button("Save...").clicked() {}
                        ui.separator();
                        if ui.button("About").clicked() {
                            self.show_about = true;
                        }
                    });
                }
                if ui.button("Edit").clicked() {
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
                }
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
            let pointer = ui.input(|i| i.pointer.interact_pos().or(i.pointer.hover_pos()));

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

            let (alloc_rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
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
            if self.primary_just_pressed && resp.drag_started() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    self.save_state();
                    if let Some(pt) = self.screen_to_canvas(pos) {
                        if self.tool == Tool::Pencil || self.tool == Tool::Eraser {
                            let r = (self.brush_size / 2.0) as i32;
                            for dy in -r..=r {
                                for dx in -r..=r {
                                    if dx * dx + dy * dy <= r * r {
                                        self.canvas.set_pixel_unchecked((pt.x + dx) as u32, (pt.y + dy) as u32, tool_color);
                                    }
                                }
                            }
                        }
                        self.draw_state = DrawState::new(pos);
                        ctx.request_repaint();
                    }
                }
            }

            // DRAG
            if matches!(self.draw_state, DrawState::Drawing { .. }) {
                if let Some(pt) = self.screen_to_canvas(pointer.unwrap_or_else(|| resp.rect.center())) {
                    let start_pt = match self.draw_state {
                        DrawState::Drawing { start } => Point::new(start.x as i32, start.y as i32),
                        _ => pt,
                    };
                    match self.tool {
                        Tool::Pencil | Tool::Eraser => {
                            shapes::draw_line(&mut self.canvas, start_pt, pt, tool_color);
                        }
                        Tool::Line => {
                            shapes::draw_dashed_line(&mut self.canvas, start_pt, pt, tool_color);
                        }
                        Tool::Rectangle => {
                            if self.fill_shape {
                                shapes::fill_rect(&mut self.canvas, start_pt, pt, tool_color);
                            } else {
                                shapes::draw_dashed_rect(&mut self.canvas, start_pt, pt, tool_color);
                            }
                        }
                        Tool::Ellipse => {
                            let cx = (start_pt.x + pt.x) / 2;
                            let cy = (start_pt.y + pt.y) / 2;
                            let rx = (pt.x - start_pt.x).unsigned_abs() as i32 / 2;
                            let ry = (pt.y - start_pt.y).unsigned_abs() as i32 / 2;
                            let center = Point::new(cx, cy);
                            if self.fill_shape {
                                shapes::fill_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                            } else {
                                shapes::draw_dashed_ellipse(&mut self.canvas, center, rx, ry, tool_color);
                            }
                        }
                        _ => {}
                    }
                    ctx.request_repaint();
                }
            }

            // RELEASE (finalize shape)
            if self.primary_just_released && matches!(self.draw_state, DrawState::Drawing { .. }) {
                if let DrawState::Drawing { start } = self.draw_state {
                    let start_pt = Point::new(start.x as i32, start.y as i32);
                    if let Some(end_pt) = self.pointer_pos.and_then(|p| self.screen_to_canvas(p)) {
                        match self.tool {
                            Tool::Line => {
                                if start.distance(egui::Pos2::new(end_pt.x as f32, end_pt.y as f32)) > 1.0 {
                                    shapes::draw_line(&mut self.canvas, start_pt, end_pt, tool_color);
                                }
                            }
                            Tool::Rectangle => {
                                if start.distance(egui::Pos2::new(end_pt.x as f32, end_pt.y as f32)) > 1.0 {
                                    if self.fill_shape {
                                        shapes::fill_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                                    } else {
                                        shapes::draw_rect(&mut self.canvas, start_pt, end_pt, tool_color);
                                    }
                                }
                            }
                            Tool::Ellipse => {
                                if start.distance(egui::Pos2::new(end_pt.x as f32, end_pt.y as f32)) > 1.0 {
                                    let cx = (start_pt.x + end_pt.x) / 2;
                                    let cy = (start_pt.y + end_pt.y) / 2;
                                    let rx = (end_pt.x - start_pt.x).unsigned_abs() as i32 / 2;
                                    let ry = (end_pt.y - start_pt.y).unsigned_abs() as i32 / 2;
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
