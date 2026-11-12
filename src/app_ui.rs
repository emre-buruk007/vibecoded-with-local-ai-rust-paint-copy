//! UI rendering for the PaintApp: menu bar, toolbar, canvas, and dialogs.

use crate::app::PaintApp;
use crate::tool::Tool;
use crate::canvas::Canvas;
use crate::draw::DrawState;
use crate::shapes::Point;
use crate::egui;

impl PaintApp {
    pub fn render_menu_bar(&mut self, ctx: &egui::Context) {
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
                    if ui.button("Undo").clicked() {
                        self.undo();
                    }
                    if ui.button("Redo").clicked() {
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
    }

    pub fn render_toolbar(&mut self, ctx: &egui::Context) -> crate::ui::toolbar::ToolbarActions {
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
        toolbar_actions.inner
    }

    pub fn render_layers_panel(&self, ctx: &egui::Context) {
        egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();
                ui.label("1 layer");
            });
    }

    pub fn render_canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Update pointer position before any mouse event handling
            self.on_pointer(ctx);

            if self.primary_just_pressed {
                self.on_mouse_press(ctx);
            }
            if let DrawState::Drawing { .. } = self.draw_state {
                self.on_mouse_drag(ctx);
            }
            if self.primary_just_released {
                self.on_mouse_release(ctx);
            }
            self.primary_just_pressed = false;
            self.primary_just_released = false;

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
            self.canvas_layout = crate::app::CanvasLayout { rect: alloc_rect, scale };

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
            self.draw_shape_preview(ui, alloc_rect, scale);
        });
    }

    fn draw_shape_preview(&self, ui: &egui::Ui, alloc_rect: egui::Rect, scale: f32) {
        if let DrawState::Drawing { start: start_pt } = self.draw_state {
            if let Some(end_pos) = self.pointer_pos {
                let local = end_pos - alloc_rect.min;
                let end_pt = Point::new((local.x / scale) as i32, (local.y / scale) as i32);
                let col = egui::Color32::from_rgba_unmultiplied(self.color.r(), self.color.g(), self.color.b(), 128);
                let off = alloc_rect.min;
                let s = scale;
                match self.tool {
                    Tool::Line => {
                        self.draw_dashed_line_painter(ui, off, start_pt, end_pt, col, s);
                    }
                    Tool::Rectangle => {
                        let x0 = off.x + start_pt.x.min(end_pt.x) as f32 * s;
                        let y0 = off.y + start_pt.y.min(end_pt.y) as f32 * s;
                        let x1 = off.x + start_pt.x.max(end_pt.x) as f32 * s;
                        let y1 = off.y + start_pt.y.max(end_pt.y) as f32 * s;
                        ui.painter().line_segment([egui::Pos2::new(x0, y0), egui::Pos2::new(x1, y0)], egui::Stroke::new(2.0_f32, col));
                        ui.painter().line_segment([egui::Pos2::new(x0, y1), egui::Pos2::new(x1, y1)], egui::Stroke::new(2.0_f32, col));
                        ui.painter().line_segment([egui::Pos2::new(x0, y0), egui::Pos2::new(x0, y1)], egui::Stroke::new(2.0_f32, col));
                        ui.painter().line_segment([egui::Pos2::new(x1, y0), egui::Pos2::new(x1, y1)], egui::Stroke::new(2.0_f32, col));
                    }
                    Tool::Ellipse => {
                        self.draw_ellipse_preview(ui, start_pt, end_pt, col, off, s);
                    }
                    _ => {}
                }
                ui.ctx().request_repaint();
            }
        }
    }

    fn draw_dashed_line_painter(&self, ui: &egui::Ui, off: egui::Pos2, p0: Point, p1: Point, color: egui::Color32, scale: f32) {
        let mut x0 = off.x + p0.x as f32 * scale;
        let mut y0 = off.y + p0.y as f32 * scale;
        let x1i = (off.x + p1.x as f32 * scale) as i32;
        let y1i = (off.y + p1.y as f32 * scale) as i32;
        let dx = (x1i - x0 as i32).unsigned_abs() as i32;
        let dy = (y1i - y0 as i32).unsigned_abs() as i32;
        let sx = if x0 < off.x + p1.x as f32 * scale { 1 } else { -1 };
        let sy = if y0 < off.y + p1.y as f32 * scale { 1 } else { -1 };
        let mut err = dx + dy;
        let mut step = 0i32;
        loop {
            if step % 2 == 0 {
                ui.painter().circle_filled(egui::Pos2::new(x0, y0), 2.0, color);
            }
            if x0 as i32 == x1i && y0 as i32 == y1i { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x0 += sx as f32; }
            if e2 <= dx { err += dx; y0 += sy as f32; }
            step += 1;
        }
    }

    fn draw_ellipse_preview(&self, ui: &egui::Ui, start: Point, end: Point, col: egui::Color32, off: egui::Pos2, scale: f32) {
        let cx = (start.x + end.x) / 2;
        let cy = (start.y + end.y) / 2;
        let rx = (end.x - start.x).abs() / 2;
        let ry = (end.y - start.y).abs() / 2;
        let mut y = -(ry as i64);
        let mut px = 0_i64;
        let mut py = (ry * ry) as i64;
        let mut px2 = 2 * rx as i64 * rx as i64;
        let mut py2 = 2 * ry as i64 * ry as i64;
        let mut p = (ry as i64 * ry as i64) - (rx as i64 * rx as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
        let mut step = 0i64;
        while px < px2 {
            if step % 2 == 0 { self.ellipse_dot(ui, cx, cy, px as i32, py as i32, col, off, scale); }
            if p < 0 { px += py2; px2 += py2; p += py2; }
            else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
            step += 1;
        }
        p = (ry as i64 * ry as i64) + (rx as i64 * rx as i64 / 4);
        while y < 0 {
            self.ellipse_dot(ui, cx, cy, px as i32, py as i32, col, off, scale);
            if p > 0 { py -= px2; py2 -= px2; p -= px2; }
            else { px += py2; px2 += py2; py -= px2; py2 -= px2; p += py2 - px2; }
            y += 1;
        }
    }

    fn ellipse_dot(&self, ui: &egui::Ui, cx: i32, cy: i32, dx: i32, dy: i32, col: egui::Color32, off: egui::Pos2, scale: f32) {
        for (px, py) in [(cx+dx,cy+dy),(cx+dx,cy-dy),(cx-dx,cy+dy),(cx-dx,cy-dy)] {
            ui.painter().circle_filled(egui::Pos2::new(off.x+px as f32*scale, off.y+py as f32*scale), 2.0, col);
        }
    }

    pub fn show_resize_dialog(&mut self, ctx: &egui::Context) {
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
    }

    pub fn show_about_dialog(&mut self, ctx: &egui::Context) {
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